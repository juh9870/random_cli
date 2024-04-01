use miette::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum NixTemplate {
    Path(PathBuf),
}

impl NixTemplate {
    pub fn generate(&self, _path: &Path) -> Result<()> {
        todo!()
    }
}
