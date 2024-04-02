use interpolator::Formattable;
use miette::{miette, Context, IntoDiagnostic};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct IdeRunner {
    pub name: Option<String>,
    /// Binary that is used to run the project
    pub binary: String,
    /// Nix package to install if the runner is missing in path
    pub nix_package: Option<String>,
    /// Format string to generate run command
    pub command: String,
}

impl IdeRunner {
    pub fn run_command(&self, path: impl AsRef<Path>) -> miette::Result<String> {
        let raw_command = self.run_command_raw(path)?;
        if which::which(&self.binary).is_ok() {
            Ok(raw_command)
        } else {
            Ok(format!(
                "nix-shell -p {} --run {}",
                self.nix_package
                    .as_ref()
                    .ok_or_else(|| miette!("Binary is missing and nix package is not set"))?,
                shell_escape::escape(Cow::Owned(raw_command))
            ))
        }
    }

    /// Returns the command required to run the IDE
    fn run_command_raw(&self, path: impl AsRef<Path>) -> miette::Result<String> {
        let path = path.as_ref();

        let path_str = path
            .to_str()
            .ok_or_else(|| miette!("Path is not a valid UTF8"))?
            .trim_end_matches('/');

        interpolator::format(
            &self.command,
            &[
                ("binary", Formattable::display(&self.binary)),
                ("path", Formattable::display(&path_str)),
            ]
            .into_iter()
            .collect::<HashMap<_, _>>(),
        )
        .into_diagnostic()
        .context("Failed to format the command")
    }
}
