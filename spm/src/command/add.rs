use crate::config::Config;
use crate::types::project::Project;
use crate::CommandError;
use clap::Args;
use crossterm::style::Stylize;
use inquire::validator::Validation;
use inquire::CustomType;
use miette::{miette, IntoDiagnostic};
use std::path::PathBuf;
use tracing::{debug_span, trace};

#[derive(Debug, Args)]
pub struct AddArgs {
    /// Path to the project
    path: Option<PathBuf>,
    /// Name of the project
    name: Option<String>,
}

impl AddArgs {
    pub fn run(self, config: &mut Config) -> Result<(), CommandError> {
        let _guard = debug_span!("Running `add` command").entered();
        let cur_dir = std::env::current_dir().into_diagnostic()?;
        let path = if let Some(path) = self.path {
            path
        } else {
            CustomType::<PathBuf> {
                message: "Project path:",
                starting_input: None,
                default: Some(cur_dir),
                placeholder: Some("Path to the project directory"),
                help_message: None,
                formatter: &|s| s.display().to_string(),
                default_value_formatter: &|s| s.display().to_string(),
                parser: &|path| match PathBuf::from(path).canonicalize() {
                    Ok(path) => Ok(path),
                    Err(_) => Err(()),
                },
                validators: vec![Box::new(&|s: &PathBuf| {
                    if s.is_dir() {
                        Ok(Validation::Valid)
                    } else {
                        Ok(Validation::Invalid(
                            "Project path must be a directory".into(),
                        ))
                    }
                })],
                error_message: "Path is invalid or points to a missing directory".to_string(),
                render_config: Default::default(),
            }
            .prompt()?
        };
        let path = path.canonicalize().into_diagnostic()?;
        let name = if let Some(name) = self.name {
            name
        } else {
            let default_name = path.file_name().map(|s| s.to_string_lossy());
            let mut text = inquire::Text::new("Project name:").with_validator(move |s: &str| {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    return Ok(Validation::Invalid("Name can not be empty".into()));
                }
                Ok(Validation::Valid)
            });
            text.default = default_name.as_deref();
            text.prompt()?
        };
        let name = name.trim().to_string();

        if name.is_empty() {
            return Err(miette!("Name can not be empty").into());
        }

        let confirm = inquire::Confirm::new(&format!(
            "Confirm adding project \"{}\" at path \"{}\"",
            name.clone().green(),
            path.display().to_string().green()
        ))
        .prompt()?;

        if !confirm {
            trace!("Canceling");
            return Err(CommandError::Canceled);
        }

        let project = Project {
            path,
            name,
            runner: None,
            environment: None,
        };

        let profile = config.get_profile_mut();
        profile.projects.insert(0, project);

        Ok(())
    }
}
