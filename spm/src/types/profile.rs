use crate::types::project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Profile {
    pub projects: Vec<Project>,
}
