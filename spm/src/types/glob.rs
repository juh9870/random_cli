use globwalk::GlobWalkerBuilder;

use miette::{Context, IntoDiagnostic};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct GlobConfig(pub String);

impl GlobConfig {
    /// Checks whenever the project directory matches the glob
    pub fn check(&self, project_dir: impl AsRef<Path>) -> miette::Result<bool> {
        let path = GlobWalkerBuilder::new(dbg!(project_dir.as_ref()), &self.0)
            .build()
            .into_diagnostic()
            .context("Invalid glob pattern")?
            .next();
        Ok(dbg!(path).is_some())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GlobEntry<T> {
    pub glob: GlobConfig,
    #[serde(flatten)]
    pub data: T,
}

pub fn first_matching<T>(
    items: &[GlobEntry<T>],
    path: impl AsRef<Path>,
) -> miette::Result<Option<&T>> {
    let path = path.as_ref();
    for item in items {
        if item.glob.check(path)? {
            return Ok(Some(&item.data));
        }
    }
    Ok(None)
}
