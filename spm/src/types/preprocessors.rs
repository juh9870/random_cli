use crate::m_try;
use crate::types::glob::GlobConfig;
use miette::{Context, IntoDiagnostic};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Preprocessor {
    pub name: String,
    pub glob: GlobConfig,
    pub command: String,
}

impl Preprocessor {
    pub fn run(&self, path: impl AsRef<Path>) -> miette::Result<()> {
        let path = path.as_ref();
        if !self.glob.check(path).unwrap() {
            return Ok(());
        }

        info!(
            name = self.name,
            command = self.command,
            "Running preprocessor"
        );

        m_try(|| {
            let mut words = shell_words::split(&self.command)
                .into_diagnostic()
                .context("Failed to parse command")?;
            std::process::Command::new(words.remove(0))
                .args(&words)
                .current_dir(path)
                .spawn()
                .into_diagnostic()
                .context("Failed to run command")?
                .wait()
                .into_diagnostic()
                .context("Preprocessor crashed")
        })
        .with_context(|| {
            format!(
                "Failed to run preprocessor {} `{}`",
                self.name, self.command
            )
        })?;

        Ok(())
    }
}
