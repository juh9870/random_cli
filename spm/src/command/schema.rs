use crate::config::Config;
use crate::CommandError;
use clap::Args;
use miette::{Context, IntoDiagnostic};
use schemars::schema_for;
use std::path::PathBuf;

#[derive(Debug, Args)]
pub struct SchemaArgs {
    /// Schema output path
    pub path: PathBuf,
}

impl SchemaArgs {
    pub fn run(self) -> Result<(), CommandError> {
        let schema = serde_json::to_string_pretty(&schema_for!(Config)).unwrap();
        fs_err::write(self.path, schema)
            .into_diagnostic()
            .context("Failed to write schema file")?;
        Ok(())
    }
}
