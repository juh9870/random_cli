use clap::{Args, Subcommand};
use cli_table::format::{Border, HorizontalLine, Justify, Separator, VerticalLine};
use cli_table::{print_stdout, Cell, Style, Table};
use inquire::validator::Validation;
use itertools::Itertools;
use miette::{miette, Context, IntoDiagnostic};
use std::borrow::Cow;
use std::collections::hash_map::Entry;
use tracing::info;
use uncased::Uncased;

use crate::config::Config;
use crate::types::profile::Profile;
use crate::CommandError;

#[derive(Debug, Args)]
pub struct ProfileArgs {
    #[command(subcommand)]
    command: ProfileSubcommands,
}

#[derive(Debug, Subcommand)]
enum ProfileSubcommands {
    /// Lists all profiles
    List,
    /// Creates a new profile
    New { name: Option<String> },
    /// Sets a profile as the default
    Default { name: Option<String> },
    /// Deletes a profile
    Delete { name: Option<String> },
}

impl ProfileArgs {
    pub fn run(self, config: &mut Config) -> Result<(), CommandError> {
        match self.command {
            ProfileSubcommands::List => list_profiles(config),
            ProfileSubcommands::New { name } => new_profile(config, name),
            ProfileSubcommands::Default { name } => default_profile(config, name),
            ProfileSubcommands::Delete { name } => delete_profile(config, name),
        }
    }
}

fn list_profiles(config: &Config) -> Result<(), CommandError> {
    let table = config
        .profiles
        .iter()
        .map(|(name, profile)| {
            vec![
                name.cell(),
                profile.projects.len().cell().justify(Justify::Right),
            ]
        })
        .collect_vec()
        .table()
        .border(
            Border::builder()
                .top(HorizontalLine::new('╭', '╮', '┬', '─'))
                .bottom(HorizontalLine::new('╰', '╯', '┴', '─'))
                .left(VerticalLine::new('│'))
                .right(VerticalLine::new('│'))
                .build(),
        )
        .separator(
            Separator::builder()
                .title(Some(HorizontalLine::new('├', '┤', '┼', '─')))
                .column(Some(VerticalLine::new('│')))
                .build(),
        )
        .title(vec![
            "ID".cell().bold(true),
            "Projects amount".cell().bold(true),
        ]);

    print_stdout(table)
        .into_diagnostic()
        .context("Failed to print profiles list")?;

    Ok(())
}

fn select_profile(config: &Config) -> Result<&str, CommandError> {
    let select =
        inquire::Select::new("Select a project", config.profiles.keys().collect_vec()).prompt()?;
    Ok(select.as_str())
}

fn find_profile(config: &Config, query: Option<String>) -> Result<Uncased<'static>, CommandError> {
    let name = if let Some(name) = query {
        Cow::Owned(name)
    } else {
        Cow::Borrowed(select_profile(config)?)
    };

    let profile_name = config.find_profile(&name)?;

    Ok(profile_name)
}

fn default_profile(config: &mut Config, name: Option<String>) -> Result<(), CommandError> {
    let profile_name = find_profile(config, name)?;

    info!(%profile_name, "Default profile changed");

    config.default_profile = profile_name;

    config.set_dirty();

    Ok(())
}

fn delete_profile(config: &mut Config, name: Option<String>) -> Result<(), CommandError> {
    let profile_name = find_profile(config, name)?;

    if config.profiles.len() == 1 {
        return Err(miette!("Cannot delete the last profile").into());
    }

    if !inquire::prompt_confirmation(&format!(
        "Are you sure you want to delete profile `{}`?",
        profile_name
    ))? {
        return Ok(());
    }

    info!(%profile_name, "Deleting profile");

    config
        .profiles
        .remove(&profile_name)
        .expect("Should have the group to delete");

    config.set_dirty();

    Ok(())
}

fn new_profile(config: &mut Config, name: Option<String>) -> Result<(), CommandError> {
    let name = if let Some(name) = name {
        Uncased::new(name.trim().to_string())
    } else {
        let profiles = config.profiles.keys().cloned().collect_vec();
        let name = inquire::Text::new("Profile name:")
            .with_validator(move |s: &str| {
                let cleaned = Uncased::new(s.trim());
                if cleaned.is_empty() {
                    return Ok(Validation::Invalid("Profile name can not be empty".into()));
                }
                if profiles.iter().any(|k| k == &cleaned) {
                    return Ok(Validation::Invalid(
                        "Profile with this name already exists".into(),
                    ));
                }
                Ok(Validation::Valid)
            })
            .prompt()?;

        Uncased::new(name)
    };

    if name.is_empty() {
        return Err(miette!("Profile name can not be empty").into());
    }

    match config.profiles.entry(name) {
        Entry::Occupied(_) => {
            return Err(miette!("Profile with this name already exists").into());
        }
        Entry::Vacant(e) => {
            e.insert(Profile { projects: vec![] });
        }
    }

    config.set_dirty();

    Ok(())
}
