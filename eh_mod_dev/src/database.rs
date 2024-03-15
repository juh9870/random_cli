use eh_schema::schema::{DatabaseItem, DatabaseItemId, DatabaseItemWithId};
use erased_serde::{Serialize, Serializer};
use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use std::ops::{Deref, Range};
use std::path::{Path, PathBuf};

use tracing::error_span;

pub trait ErasedDbItem: Serialize + Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any + Serialize> ErasedDbItem for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub fn borrow_downcast<T: Any>(cell: &RefCell<dyn Any>) -> Option<Ref<T>> {
    let r = cell.borrow();
    if (*r).type_id() == TypeId::of::<T>() {
        Some(Ref::map(r, |x| x.downcast_ref::<T>().unwrap()))
    } else {
        None
    }
}

pub fn borrow_downcast_mut<T: Any>(cell: &RefCell<dyn Any>) -> Option<RefMut<T>> {
    let r = cell.borrow_mut();
    if (*r).type_id() == TypeId::of::<T>() {
        Some(RefMut::map(r, |x| x.downcast_mut::<T>().unwrap()))
    } else {
        None
    }
}

const MAPPINGS_NAME: &str = "id_mappings.json5";
const MAPPINGS_BACKUP_NAME: &str = "id_mappings.json5.backup";

pub struct Database {
    pub path: PathBuf,
    ids: RefCell<HashMap<String, HashMap<String, i32>>>,
    used_ids: RefCell<HashMap<String, HashSet<i32>>>,
    available_ids: RefCell<HashMap<TypeId, Vec<Range<i32>>>>,
    default_ids: Vec<Range<i32>>,
    items: HashMap<TypeId, (&'static str, HashMap<i32, RefCell<Box<dyn ErasedDbItem>>>)>,
}

fn check_no_backup(path: &Path) {
    let _guard =
        error_span!("Checking for mapping backup file presence", path=%path.display()).entered();
    if path.exists() {
        panic!("Mappings backup file exists, this means that there was an error during the previous invocation, please manually check your ID files to avoid data corruption. Remove the file after data integrity is ensured.")
    }
}

impl Database {
    /// Constructs a new database builder. Don't forget to allocate ID space
    /// via [add_id_range] or [add_id_range_for] methods
    ///
    /// # Panics
    /// Will panic if output path contains a mappings file but it can't be read or invalid
    ///
    /// Will panic if mappings backup exists
    pub fn new(output_path: impl AsRef<Path>) -> Self {
        let output_path = std::env::current_dir()
            .expect("Should be able to get current directory info from process env")
            .join(output_path);

        let _guard =
            error_span!("Creating a new database", output_path=%output_path.display()).entered();
        if !output_path.exists() {
            panic!("Target directory does not exist")
        }

        let mappings_path = output_path.join(MAPPINGS_NAME);
        let mappings: RefCell<HashMap<String, HashMap<String, i32>>> = mappings_path
            .exists()
            .then(|| {
                let data = fs_err::read_to_string(output_path.join(MAPPINGS_NAME))
                    .expect("Should be able to read mappings file");
                serde_json5::from_str(&data).expect("Should be able to deserialize mappings file")
            })
            .unwrap_or_default();

        check_no_backup(&output_path.join(MAPPINGS_BACKUP_NAME));

        let used_ids = RefCell::new(
            mappings
                .borrow()
                .iter()
                .map(|(k, v)| ((k.clone(), v.values().copied().collect::<HashSet<i32>>())))
                .collect(),
        );

        Self {
            path: output_path.to_path_buf(),
            used_ids,
            ids: mappings,
            available_ids: Default::default(),
            default_ids: Default::default(),
            items: Default::default(),
        }
    }

    /// Adds another ID range to use for all types
    pub fn add_id_range(&mut self, range: Range<i32>) {
        for ids in self.available_ids.get_mut().values_mut() {
            ids.push(range.clone());
        }
        self.default_ids.push(range);
    }

    /// Adds another ID range to use for one specified type
    pub fn add_id_range_for<T: 'static + DatabaseItem>(&mut self, range: Range<i32>) {
        let type_id = TypeId::of::<T>();
        self.available_ids
            .get_mut()
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
            .get_mut()
            .entry(type_id)
            .or_insert_with(|| self.default_ids.clone())
            .push(range)
    }

    /// Converts string ID into database item ID
    ///
    /// Aborts the execution if generating ID is not possible
    pub fn id<T: 'static + DatabaseItem>(&self, id: impl Into<String>) -> DatabaseItemId<T> {
        let id_str = id.into();
        let _guard = error_span!("Getting item ID", id = id_str, ty = T::type_name()).entered();

        let type_name = T::type_name();
        let mut ids_borrow = self.ids.borrow_mut();
        let mapping = ids_borrow.entry(type_name.to_string()).or_default();

        match mapping.get(&id_str) {
            None => {
                drop(ids_borrow);
                let id = self.next_id_raw::<T>();
                self.ids
                    .borrow_mut()
                    .get_mut(type_name)
                    .expect("ID entry should be present at this point")
                    .insert(id_str, id);
                id.into()
            }
            Some(id) => (*id).into(),
        }
    }

    /// Forcefully assigns numeric ID to a string
    pub fn set_id<T: 'static + DatabaseItem>(
        &self,
        string_id: impl Into<String>,
        numeric_id: i32,
    ) -> DatabaseItemId<T> {
        let type_name = T::type_name();
        let mut ids_borrow = self.ids.borrow_mut();
        let mut used_ids_borrow = self.used_ids.borrow_mut();
        ids_borrow
            .entry(type_name.to_string())
            .or_default()
            .insert(string_id.into(), numeric_id);
        used_ids_borrow
            .entry(type_name.to_string())
            .or_default()
            .insert(numeric_id);
        DatabaseItemId::new(numeric_id)
    }

    /// Gets the immutable reference to the item that was previously recorded in database
    ///
    /// # Panics
    /// All items are stored behind a [RefCell], so regular runtime borrowing rules apply
    pub fn get_item<T: 'static + DatabaseItem>(
        &self,
        id: impl DatabaseIdLike<T>,
    ) -> Option<Ref<T>> {
        let id = id.into_id(self);
        let _guard = error_span!("Getting item", id = id.0, ty = T::type_name()).entered();

        let type_id = TypeId::of::<T>();

        self.items.get(&type_id).and_then(|mapping| {
            mapping.1.get(&id.0).map(|item| {
                Ref::map(item.borrow(), |r| {
                    r.as_any()
                        .downcast_ref::<T>()
                        .expect("Only item of the valid type should be stored in the map")
                })
                // item.borrow()
                //     .as_any()
                //     .downcast_ref()
                //     .expect("Only item of the valid type should be stored in the map")
            })
        })
    }

    /// Gets the mutable reference to the item that was previously recorded in database
    ///
    /// # Panics
    /// All items are stored behind a [RefCell], so regular runtime borrowing rules apply
    pub fn get_item_mut<T: 'static + DatabaseItem>(
        &self,
        id: impl DatabaseIdLike<T>,
    ) -> Option<RefMut<T>> {
        let id = id.into_id(self);
        let _guard = error_span!("Getting item (mut)", id = id.0, ty = T::type_name()).entered();

        let type_id = TypeId::of::<T>();

        self.items.get(&type_id).and_then(|mapping| {
            mapping.1.get(&id.0).map(|item| {
                RefMut::map(item.borrow_mut(), |r| {
                    r.as_any_mut()
                        .downcast_mut::<T>()
                        .expect("Only item of the valid type should be stored in the map")
                })
                // item.borrow_mut()
                //     .as_any_mut()
                //     .downcast_mut()
                //     .expect("Only item of the valid type should be stored in the map")
            })
        })
    }
    /// Adds an item to the database, using manually provided ID
    ///
    /// Returns a mutable reference to the inserted item
    ///
    /// # Panics
    /// All items are stored behind a [RefCell], so regular runtime borrowing rules apply
    pub fn add_item<T: 'static + DatabaseItemWithId + Any>(
        &mut self,
        item: T,
    ) -> DbInsertionResponse<T> {
        self.add_item_manual(item.id(), item)
    }

    /// Adds an item to the database, using manually provided ID
    ///
    /// Returns a mutable reference to the inserted item
    ///
    /// # Panics
    /// All items are stored behind a [RefCell], so regular runtime borrowing rules apply
    pub fn add_item_manual<T: 'static + DatabaseItem + Any>(
        &mut self,
        id: impl DatabaseIdLike<T>,
        item: T,
    ) -> DbInsertionResponse<T> {
        let id = id.into_id(self);
        let _guard = error_span!("Inserting item", id = id.0, ty = T::type_name()).entered();

        let type_id = TypeId::of::<T>();
        let mapping = self
            .items
            .entry(type_id)
            .or_insert_with(|| (T::type_name(), Default::default()));

        match mapping.1.entry(id.0) {
            Entry::Occupied(_) => {
                panic!("Item with this ID and type is already added");
            }
            Entry::Vacant(entry) => entry.insert(RefCell::new(Box::new(item))),
        };

        DbInsertionResponse { id }
    }

    /// Saves database to the file system, overriding old files
    pub fn save(mut self) {
        let path = self.path;
        let _guard = error_span!("Saving database", path=%path.display()).entered();

        let path = path
            .canonicalize()
            .expect("Should be able to canonicalize path");

        if !path.is_dir() {
            panic!("Output path is not a directory");
        }

        let mut saved_files = HashSet::new();

        let mappings_path = path.join(MAPPINGS_NAME);
        let mappings_bk_path = path.join(MAPPINGS_BACKUP_NAME);
        check_no_backup(&mappings_bk_path);

        let code =
            serde_json::to_string_pretty(&self.ids).expect("Should be able to serialize mappings");

        if mappings_path.exists() {
            fs_err::copy(&mappings_path, &mappings_bk_path)
                .expect("Should be able to create mappings backup");
            fs_err::write(&mappings_path, code).expect("Should be able to write mappings file");
        } else {
            fs_err::write(&mappings_path, code).expect("Should be able to write mappings file");
            fs_err::copy(&mappings_path, &mappings_bk_path)
                .expect("Should be able to create mappings backup");
        }

        saved_files.insert(mappings_path);
        saved_files.insert(mappings_bk_path.clone());

        for (_, (type_name, files)) in self.items {
            let id_name = self.ids.get_mut().entry(type_name.to_string()).or_default();
            let inverse: HashMap<_, _> = id_name.iter().map(|(k, v)| (*v, k.clone())).collect();
            for (id, file) in files {
                let id = inverse
                    .get(&id)
                    .cloned()
                    .unwrap_or_else(|| format!("auto_{}", id))
                    .replace(':', "-");

                let file_name = format!("{id}_{type_name}.json");

                let path = path.join(file_name);

                let _guard = error_span!("Saving file", path=%path.display()).entered();

                if saved_files.contains(&path) {
                    panic!("File with this name was already saved");
                }

                let mut buf: Vec<u8> = vec![];
                let mut json_serializer = serde_json::Serializer::pretty(&mut buf);
                let mut json = Box::new(<dyn Serializer>::erase(&mut json_serializer));

                file.into_inner()
                    .erased_serialize(&mut json)
                    .expect("Should be able to serialize an item");
                drop(json);
                fs_err::write(&path, buf).expect("Should be able to write the file");

                saved_files.insert(path);
            }
        }

        let files = fs_err::read_dir(path).expect("Should be able to read output directory");
        for file in files {
            let file = file.expect("Should be able to read a dir entry");
            let _guard = error_span!("Checking entry", path=%file.path().display()).entered();
            if !file.path().is_file() {
                panic!("Output directory contains a non-file entry");
            }

            if saved_files.contains(&file.path()) {
                continue;
            }

            let _guard = error_span!("Cleaning up old file", path=%file.path().display()).entered();

            fs_err::remove_file(file.path()).expect("Should be able to delete old file");
        }

        fs_err::remove_file(mappings_bk_path).expect("Should remove mappings backup file");
    }

    fn next_id_raw<T: 'static + DatabaseItem>(&self) -> i32 {
        if self.default_ids.is_empty() {
            panic!(
                "No ID range were given for Database to assign, please use `add_id_range` method"
            )
        }

        let type_id = TypeId::of::<T>();
        let type_name = T::type_name();
        let mut available_ids_borrow = self.available_ids.borrow_mut();
        let ids = available_ids_borrow
            .entry(type_id)
            .or_insert_with(|| self.default_ids.clone());

        let mut ids_borrow = self.used_ids.borrow_mut();

        let mappings = ids_borrow.entry(type_name.to_string()).or_default();

        while let Some(id) = ids.iter_mut().find_map(|range| range.next()) {
            // Check that ID is not already in use
            if !mappings.contains(&id) {
                mappings.insert(id);
                return id;
            }
        }

        panic!("No free IDs are left for this type");
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

pub trait DatabaseIdLike<T: 'static + DatabaseItem> {
    fn into_id(self, database: &Database) -> DatabaseItemId<T>;
}

impl<T: 'static + DatabaseItem> DatabaseIdLike<T> for DatabaseItemId<T> {
    fn into_id(self, _database: &Database) -> DatabaseItemId<T> {
        self
    }
}

impl<T: 'static + DatabaseItem> DatabaseIdLike<T> for &str {
    fn into_id(self, database: &Database) -> DatabaseItemId<T> {
        database.id(self)
    }
}

impl<T: 'static + DatabaseItem> DatabaseIdLike<T> for String {
    fn into_id(self, database: &Database) -> DatabaseItemId<T> {
        database.id(self)
    }
}

pub struct DbInsertionResponse<T: 'static + DatabaseItem> {
    id: DatabaseItemId<T>,
}

impl<T: 'static + DatabaseItem> DbInsertionResponse<T> {
    pub fn item(self, db: &Database) -> Ref<T> {
        db.get_item::<T>(self.id).unwrap()
    }
    pub fn item_mut(self, db: &Database) -> RefMut<T> {
        db.get_item_mut::<T>(self.id).unwrap()
    }

    pub fn id(self) -> DatabaseItemId<T> {
        self.id
    }
}

impl<T: 'static + DatabaseItem> DatabaseIdLike<T> for DbInsertionResponse<T> {
    fn into_id(self, _database: &Database) -> DatabaseItemId<T> {
        self.id
    }
}
