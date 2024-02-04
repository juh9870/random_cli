use std::collections::BTreeMap;
use std::iter::once;
use std::path::{Path, PathBuf};

use ahash::{AHashMap, AHashSet};
use btreeprint::adapter::display_iter;
use clap::Parser;
use color_eyre::eyre::{bail, eyre, Context, ContextCompat};
use color_eyre::owo_colors::OwoColorize;
use git2::{DiffDelta, Repository};
use ignore::gitignore::Gitignore;
use itertools::Itertools;
use strum::EnumIs;
use tracing::{debug, error_span};
use tracing_error::ErrorLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;

/// pre-commit hook for preventing accidentally committing debug/sensitive data or changes.
#[derive(Parser, Debug)]
struct Args {
    /// Pattern which is used to detect "forbidden" changes
    #[arg(long, default_value = "#nocommit")]
    content_pattern: String,

    /// Name of the marker file used to mark whole directories as "forbidden"
    #[arg(long, default_value = ".nocommit")]
    marker_filename: String,
}

fn main() -> color_eyre::Result<()> {
    let subscriber = tracing_subscriber::Registry::default()
        .with(tracing_subscriber::fmt::Layer::default())
        .with(EnvFilter::from_default_env())
        .with(ErrorLayer::default());
    tracing::subscriber::set_global_default(subscriber).unwrap();
    color_eyre::install()?;

    let Args {
        content_pattern,
        marker_filename,
    } = Args::parse();
    let git_repo = Repository::discover(Path::canonicalize("./".as_ref())?)
        .context("Discovering git repository")?;

    let mut repository_root = git_repo.path().canonicalize()?;

    if repository_root.ends_with(".git") {
        repository_root.pop();
    }

    let bypass_file = repository_root.join(".nocommitignore");
    let ignore_pattern = if bypass_file.is_file() {
        let (gi, err) = Gitignore::new(bypass_file);
        if let Some(err) = err {
            return Err(err.into());
        }
        Some(gi)
    } else {
        None
    };

    let mut state = NocommitState::new(
        content_pattern,
        marker_filename,
        repository_root.clone(),
        ignore_pattern,
    );

    let changed_files = changed_files(&git_repo)?;
    debug!(changed_files = ?changed_files);
    for (path, ty) in changed_files {
        state.validate_file(path.canonicalize()?, ty)?;
    }
    if state.banned_content.is_empty() && state.banned_files.is_empty() {
        return Ok(());
    }
    let mut errors = BTreeMap::new();

    for (folder, files) in state.banned_files {
        errors.insert(folder, "folder is marked as nocommit");
        for file in files {
            errors.insert(file, "file is in nocommit folder");
        }
    }
    for x in state.banned_content {
        errors.insert(x, "file contains nocommit pattern");
    }
    let (paths, messages): (Vec<_>, Vec<_>) = errors.into_iter().unzip();

    let errors = display_iter::<str, _>(
        once(state.repository_root.to_string_lossy() + "/").chain(
            paths
                .iter()
                .map(|e| e.to_string_lossy() + if e.is_dir() { "/" } else { "" }),
        ),
    )
    .zip(once("In this repository").chain(messages))
    .map(|(path, msg)| format!("{} - {}", path, msg.red()))
    .join("\n");

    println!(
        "{}\n{}",
        "Commit was blocked due to forbidden files".red(),
        errors
    );

    std::process::exit(1);
}

#[derive(Debug)]
struct NocommitState {
    content_pattern: String,
    marker_filename: String,

    repository_root: PathBuf,

    banned_content: AHashSet<PathBuf>,
    folders: AHashMap<PathBuf, FolderStatus>,
    banned_files: AHashMap<PathBuf, AHashSet<PathBuf>>,
    ignore_pattern: Option<Gitignore>,
}

impl NocommitState {
    pub fn new(
        content_pattern: String,
        marker_filename: String,
        mut repository_root: PathBuf,
        pattern: Option<Gitignore>,
    ) -> Self {
        if repository_root.ends_with(".git") {
            repository_root.pop();
        }
        Self {
            content_pattern,
            marker_filename,
            repository_root,
            banned_content: Default::default(),
            folders: Default::default(),
            banned_files: Default::default(),
            ignore_pattern: pattern,
        }
    }
}

#[derive(Debug, Clone, EnumIs)]
enum FolderStatus {
    Allowed,
    Banned(PathBuf),
}

#[derive(Debug, Clone, EnumIs)]
enum ChangedFile {
    Text,
    Binary,
}

fn changed_files(repo: &Repository) -> color_eyre::Result<Vec<(PathBuf, ChangedFile)>> {
    let _guard = error_span!("Detecting changed files").entered();

    let head_tree = repo.head()?.peel_to_tree()?;
    let diff = repo.diff_tree_to_index(Some(&head_tree), None, None)?;
    let mut changes = Vec::new();
    diff.foreach(
        &mut Box::new(|dd: DiffDelta, _f: f32| {
            let df = dd.new_file();
            if !df.exists() {
                return true;
            }

            let Some(path) = df.path() else {
                return false;
            };
            changes.push((
                path.to_path_buf(),
                if df.is_binary() {
                    ChangedFile::Binary
                } else {
                    ChangedFile::Text
                },
            ));
            true
        }),
        None,
        None,
        None,
    )
    .map_err(|_| eyre!("Failed to fetch path info for one of the changed files"))?;

    Ok(changes)
}

impl NocommitState {
    fn validate_file(
        &mut self,
        path: impl AsRef<Path>,
        file_type: ChangedFile,
    ) -> color_eyre::Result<()> {
        let path = path.as_ref();
        let _guard = error_span!("Validating a file", path = %path.display()).entered();

        if self
            .ignore_pattern
            .as_ref()
            .map(|e| e.matched(path, false).is_ignore())
            .unwrap_or(false)
        {
            return Ok(());
        }

        if file_type.is_text() {
            let content = fs_err::read_to_string(path)?;
            if content.contains(&self.content_pattern) {
                self.banned_content.insert(path.to_path_buf());
            }
        }

        if let FolderStatus::Banned(folder) =
            self.validate_dir(path.parent().context("Should have parent")?)?
        {
            self.banned_files
                .entry(folder)
                .or_default()
                .insert(path.to_path_buf());
        }

        Ok(())
    }

    fn validate_dir(&mut self, path: &Path) -> color_eyre::Result<FolderStatus> {
        let _guard2 = error_span!("Validating directory", path = %path.display()).entered();
        if let Some(v) = self.folders.get(path) {
            return Ok(v.clone());
        }
        if !path.is_dir() {
            bail!("Not a directory")
        }
        if path != self.repository_root {
            if !path.starts_with(&self.repository_root) {
                bail!(
                    "Somehow escaped from the repository root. {} is not in {}",
                    path.display(),
                    self.repository_root.display()
                )
            }
            let parent_status = self.validate_dir(
                path.parent()
                    .context("Paths should have parent at this point")?,
            )?;
            if parent_status.is_banned() {
                let status = parent_status.clone();
                self.folders.insert(path.to_path_buf(), status.clone());
                return Ok(status);
            }
        }
        if path.join(&self.marker_filename).exists() {
            self.folders
                .insert(path.to_path_buf(), FolderStatus::Banned(path.to_path_buf()));
            return Ok(FolderStatus::Banned(path.to_path_buf()));
        }
        self.folders
            .insert(path.to_path_buf(), FolderStatus::Allowed);

        Ok(FolderStatus::Allowed)
    }
}
