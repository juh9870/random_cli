use crate::types::ide_runner::IdeRunner;
use crate::types::template::NixTemplate;
use interpolator::Formattable;
use miette::{miette, Context, IntoDiagnostic};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::path::Path;

/// Environment the project is running in
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProjectEnv {
    pub name: Option<String>,
    /// Template file for this environment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nix_template: Option<NixTemplate>,
    /// Runner that is used to run the IDE
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runner: Option<IdeRunner>,
    /// Format string to generate run command
    pub command: String,
    /// Format string to generate run command
    pub ide_command: String,
}

impl ProjectEnv {
    pub fn run_command(
        &self,
        ide: Option<&IdeRunner>,
        path: impl AsRef<Path>,
    ) -> miette::Result<String> {
        let path = path.as_ref();
        let path_str = path
            .to_str()
            .ok_or_else(|| miette!("Path is not a valid UTF8"))?
            .trim_end_matches('/');

        let command = if let Some(ide) = ide {
            let ide_command = ide
                .run_command(path)
                .context("Failed to generate IDE run command")?;
            interpolator::format(
                &self.ide_command,
                &[
                    (
                        "'ide'",
                        Formattable::display(&format!(
                            "{}",
                            shell_escape::escape(Cow::Borrowed(&ide_command))
                        )),
                    ),
                    ("ide", Formattable::display(&ide_command)),
                    ("path", Formattable::display(&path_str)),
                ]
                .into_iter()
                .collect::<HashMap<_, _>>(),
            )
            .into_diagnostic()
            .context("Failed to format the command")?
        } else {
            interpolator::format(
                &self.command,
                &[("path", Formattable::display(&path_str))]
                    .into_iter()
                    .collect::<HashMap<_, _>>(),
            )
            .into_diagnostic()
            .context("Failed to format the command")?
        };

        Ok(command)
    }
}
