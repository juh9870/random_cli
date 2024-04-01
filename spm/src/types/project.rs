use crate::types::ide_runner::IdeRunner;
use crate::types::project_env::ProjectEnv;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Project {
    pub path: PathBuf,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runner: Option<IdeRunner>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<ProjectEnv>,
}
