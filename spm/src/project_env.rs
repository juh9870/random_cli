use crate::ide_runner::IdeRunner;
use crate::template::NixTemplate;
use globwalk::GlobWalkerBuilder;
use miette::{Context, IntoDiagnostic, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Environment the project is running in
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectEnv {
    /// Glob used to match projects against this env
    pub glob: Option<String>,
    /// Template file for this environment
    pub nix_template: Option<NixTemplate>,
    /// Runner that is used to run the IDE
    pub runner: Option<IdeRunner>,
}

impl ProjectEnv {
    /// Checks whenever the current directory matches the references
    pub fn check(&self, project_dir: &Path) -> Result<bool> {
        if let Some(glob_pattern) = &self.glob {
            Ok(GlobWalkerBuilder::new(project_dir, glob_pattern)
                .build()
                .into_diagnostic()
                .context("Invalid glob pattern")?
                .next()
                .is_some())
        } else {
            Ok(false)
        }
    }
}
