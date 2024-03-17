use miette::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NixTemplate {
    Path(PathBuf),
}

impl NixTemplate {
    pub fn generate(_path: &Path) -> Result<()> {
        todo!()
    }
}
