use crate::config::Config;
use crate::errors::TooManyErr;
use crate::project::Project;
use crate::CommandError;
use clap::Args;
use crossterm::style::Stylize;
use itertools::Itertools;
use miette::miette;

#[derive(Debug, Args)]
pub struct RunArgs {
    /// Name of the project to run
    name: Option<String>,
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

        let _project = if let Some(project) = project {
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

        Ok(())
    }
}
