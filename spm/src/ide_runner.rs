use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdeRunner {
    /// Binary that is used to run the project
    pub binary: String,
    /// Nix package to install if the runner is missing in path
    pub nix_package: Option<String>,
    /// Format string to generate run command
    pub fmt: Option<String>,
}

impl IdeRunner {}
