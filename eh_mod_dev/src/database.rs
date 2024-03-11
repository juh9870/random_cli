use bimap::BiMap;
use eh_schema::schema::{DatabaseItem, DatabaseItemId};
use erased_serde::{Serialize, Serializer};
use std::any::{Any, TypeId};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::ops::{Deref, Range};
use std::path::{Path, PathBuf};
use std::process::exit;
use tracing::{error, error_span};

fn abort() -> ! {
    exit(1);
}

trait ErasedDbItem: Serialize + Any {}

impl<T: Any + Serialize> ErasedDbItem for T {}

pub struct Database {
    pub path: PathBuf,
    ids: HashMap<TypeId, BiMap<String, i32>>,
    available_ids: HashMap<TypeId, Vec<Range<i32>>>,
    default_ids: Vec<Range<i32>>,
    items: HashMap<TypeId, (&'static str, HashMap<i32, Box<dyn Serialize>>)>,
}

impl Database {
    /// Constructs a new database builder. Don't forget to allocate ID space
    /// via [add_id_range] or [add_id_range_for] methods
    pub fn new() -> Self {
        Self {
            path: Default::default(),
            ids: Default::default(),
            available_ids: Default::default(),
            default_ids: vec![],
            items: Default::default(),
        }
    }

    /// Adds another ID range to use for all types
    pub fn add_id_range(&mut self, range: Range<i32>) {
        for ids in self.available_ids.values_mut() {
            ids.push(range.clone());
        }
        self.default_ids.push(range);
    }

    /// Adds another ID range to use for one specified type
    pub fn add_id_range_for<T: 'static + DatabaseItem>(&mut self, range: Range<i32>) {
        let type_id = TypeId::of::<T>();
        self.available_ids
            .entry(type_id)
            .or_insert_with(|| self.default_ids.clone())
            .push(range)
    }

    /// Clears allocated ID ranges for the specified type, preventing obtaining
    /// of new IDs until [add_id_range] or [add_id_range_for] are used to
    /// allocate new ID space for this type
    pub fn clear_id_ranges_for<T: 'static + DatabaseItem>(&mut self, range: Range<i32>) {
        let type_id = TypeId::of::<T>();
        self.available_ids
            .entry(type_id)
            .or_insert_with(|| self.default_ids.clone())
            .push(range)
    }

    /// Converts string ID into database item ID
    ///
    /// Aborts the execution if generating ID is not possible
    pub fn id<T: 'static + DatabaseItem>(&mut self, id: impl Into<String>) -> DatabaseItemId<T> {
        let id_str = id.into();
        let _guard = error_span!("Getting item ID", id = id_str, ty = T::type_name()).entered();

        let type_id = TypeId::of::<T>();
        let mapping = self.ids.entry(type_id).or_default();

        match mapping.get_by_left(&id_str) {
            None => {
                let id = self.next_id_raw(type_id);
                self.ids.get_mut(&type_id).unwrap().insert(id_str, id);
                id.into()
            }
            Some(id) => (*id).into(),
        }
    }

    /// Gets the immutable reference to the item that was previously recorded in database
    pub fn get_item<T: 'static + DatabaseItem>(&self, id: DatabaseItemId<T>) -> Option<&T> {
        let _guard = error_span!("Getting item", id = id.0, ty = T::type_name()).entered();

        let type_id = TypeId::of::<T>();

        self.items.get(&type_id).and_then(|mapping| {
            mapping.1.get(&id.0).map(|item| {
                <dyn Any>::downcast_ref::<T>(item)
                    .expect("Only item of the valid type should be stored in the map")
            })
        })
    }

    /// Gets the mutable reference to the item that was previously recorded in database
    pub fn get_item_mut<T: 'static + DatabaseItem>(
        &mut self,
        id: DatabaseItemId<T>,
    ) -> Option<&mut T> {
        let _guard = error_span!("Getting item", id = id.0, ty = T::type_name()).entered();

        let type_id = TypeId::of::<T>();
        let mapping = self.items.entry(type_id).or_default();

        mapping.1.get_mut(&id.0).map(|item| {
            <dyn Any>::downcast_mut::<T>(item)
                .expect("Only item of the valid type should be stored in the map")
        })
    }

    /// Adds an item to the database
    pub fn add_item<T: 'static + DatabaseItem + Any>(&mut self, id: DatabaseItemId<T>, item: T) {
        let _guard = error_span!("Inserting item", id = id.0, ty = T::type_name()).entered();

        let type_id = TypeId::of::<T>();
        let mapping = self
            .items
            .entry(type_id)
            .or_insert_with(|| (T::type_name(), Default::default()));

        match mapping.1.entry(id.0) {
            Entry::Occupied(_) => {
                error!("Item with this ID and type is already added");
                abort();
            }
            Entry::Vacant(entry) => entry.insert(Box::new(item)),
        };
    }

    /// Saves database to the file system, overriding old files
    pub fn save(mut self) {
        let path = self.path;
        let _guard = error_span!("Saving database", path=%path.display()).entered();

        let path = path.canonicalize().unwrap();

        if !path.is_dir() {
            error!("Output path is not a directory");
            abort();
        }

        let mut saved_files = HashSet::new();

        saved_files.insert(path.join("ids.mappings"));

        for (type_id, (type_name, files)) in self.items {
            let id_name = self.ids.entry(type_id).or_default();
            for (id, file) in files {
                let id = id_name
                    .get_by_right(&id)
                    .cloned()
                    .unwrap_or_else(|| format!("auto_{}", id));

                let file_name = format!("{id}_{type_name}.json");

                let path = path.join(file_name);

                let _guard = error_span!("Saving file", path=%path.display()).entered();

                if saved_files.contains(&path) {
                    error!("File with this name was already saved");
                    abort();
                }

                let mut buf: Vec<u8> = vec![];
                let mut json_serializer = serde_json::Serializer::pretty(&mut buf);
                let mut json = Box::new(<dyn Serializer>::erase(&mut json_serializer));

                <dyn Serialize>::erased_serialize(&file, &mut json).unwrap();
                drop(json);
                fs_err::write(&path, buf).unwrap();

                saved_files.insert(path);
            }
        }

        let files = fs_err::read_dir(path).unwrap();
        for file in files {
            let file = file.unwrap();
            if !file.path().is_file() {
                error!(path=%file.path().display(), "Output directory contains a non-file entry");
                abort();
            }

            if saved_files.contains(&file.path()) {
                continue;
            }

            let _guard = error_span!("Cleaning up old file", path=%file.path().display()).entered();

            fs_err::remove_file(file.path()).unwrap();
        }
    }

    fn next_id_raw(&mut self, type_id: TypeId) -> i32 {
        if self.default_ids.is_empty() {
            error!(
                "No ID range were given for Database to assign, please use `add_id_range` method"
            )
        }
        let ids = self
            .available_ids
            .entry(type_id)
            .or_insert_with(|| self.default_ids.clone());

        let mappings = self.ids.entry(type_id).or_default();

        while let Some(id) = ids.iter_mut().find_map(|range| range.next()) {
            // Check that ID is not already in use
            if !mappings.contains_right(&id) {
                return id;
            }
        }

        error!("No free IDs are left for this type");
        abort();
    }
}

#[derive(Debug, Clone)]
struct Readonly<T>(T);

impl<T> Deref for Readonly<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> AsRef<T> for Readonly<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}
