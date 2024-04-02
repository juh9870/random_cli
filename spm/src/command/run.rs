use crate::config::Config;
use crate::errors::TooManyErr;
use crate::types::glob::first_matching;
use crate::types::project::Project;
use crate::{m_try, CommandError};
use clap::Args;
use crossterm::style::Stylize;
use itertools::Itertools;
use miette::{miette, Context, IntoDiagnostic};
use tracing::{info, warn};

#[derive(Debug, Args, Default)]
pub struct RunArgs {
    /// Name of the project to run
    name: Option<String>,
    /// Attach to the IDE
    #[arg(long, short)]
    attach: bool,
}

impl RunArgs {
    pub fn run(self, config: &mut Config) -> Result<(), CommandError> {
        let profile = config.get_profile();

        if profile.projects.is_empty() {
            println!("{}", "No projects are added yet".dark_yellow());
            return Ok(());
        }

        let mut project: Option<&Project> = None;
        if let Some(name) = self.name {
            let projects = profile
                .projects
                .iter()
                .filter(|p| &p.name == &name)
                .collect_vec();
            match projects.len() {
                0 => return Err(miette!("{}", "Multiple matching projects are found").into()),
                1 => {
                    project = Some(projects.into_iter().next().unwrap());
                }
                _ => {
                    return Err(TooManyErr {
                        query: "Multiple matching projects are found".to_string(),
                        options: projects
                            .into_iter()
                            .map(|p| p.name.to_string())
                            .collect_vec(),
                    }
                    .into());
                }
            }
        }

        let longest_name = profile.projects.iter().map(|p| p.name.len()).max().unwrap();
        let names = profile
            .projects
            .iter()
            .map(|p| format!("{: <longest_name$}    {}", p.name, p.path.display()))
            .collect_vec();

        let project = if let Some(project) = project {
            project
        } else {
            let select = inquire::Select::new("Select a project", names).raw_prompt()?;
            let idx = select.index;
            if idx != 0 {
                let profile = config.get_profile_mut();
                let item = profile.projects.remove(idx);
                profile.projects.insert(0, item);
            }

            let profile = config.get_profile();
            &profile.projects[0]
        };

        let env = if let Some(env) = &project.environment {
            env
        } else {
            let Some(env) = first_matching(&config.environments, &project.path)
                .context("Failed to find matching environment")?
            else {
                return Err(miette!("{}", "No valid environment found").into());
            };
            env
        };

        info!(
            "Environment detected: {}",
            env.name.as_deref().unwrap_or("Unknown")
        );

        let run_ide = inquire::Confirm::new("Run IDE?")
            .with_default(true)
            .prompt()?;

        let ide = if run_ide {
            let ide = if let Some(ide) = &project.runner {
                ide
            } else if let Some(ide) = &env.runner {
                ide
            } else {
                let Some(ide) = first_matching(&config.runners, &project.path)
                    .context("Failed to find matching IDE runner")?
                else {
                    return Err(miette!("{}", "No applicable IDE found").into());
                };
                ide
            };

            info!("IDE detected: {}", ide.name.as_deref().unwrap_or("Unknown"));
            Some(ide)
        } else {
            None
        };

        let mut run_command = env.run_command(ide, &project.path)?;

        for preprocessor in &config.preprocessors {
            preprocessor.run(&project.path)?;
        }

        let unit_name = format!("spm-{}-{}", config.current_profile_name(), project.name);

        if run_ide && !self.attach {
            if which::which("screen").is_ok() {
                info!("Running with screen");
                run_command = format!("screen -dmS {} {}", unit_name, run_command);
            } else if which::which("systemd-run").is_ok() {
                info!("Running with systemd-run");
                run_command = format!("systemd-run --user --unit={} {}", unit_name, run_command);
            } else if which::which("nohup").is_ok() {
                info!("Running with nohup");
                run_command = format!("nohup {} &", run_command);
            } else {
                warn!("No background runner is found, running in foreground.");
                info!("Supported background runners: screen, systemd-run, nohup")
            }
        }

        info!(name = project.name, run_command, "Running project");

        m_try(|| {
            let mut words = shell_words::split(&run_command)
                .into_diagnostic()
                .context("Failed to parse command")?;
            std::process::Command::new(words.remove(0))
                .args(&words)
                .current_dir(&project.path)
                .spawn()
                .into_diagnostic()
                .context("Failed to run command")?
                .wait()
                .into_diagnostic()
                .context("IDE crashed")
        })
        .with_context(|| format!("Failed to run command `{run_command}`"))?;

        Ok(())
    }
}
