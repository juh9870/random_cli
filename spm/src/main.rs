use crate::command::add::AddArgs;
use crate::config::Config;
use clap::{Args, Parser, Subcommand};

use inquire::InquireError;

use miette::{miette, Report, Result};

use clap_verbosity_flag::Verbosity;
use crossterm::style::Stylize;
use std::path::PathBuf;
use thiserror::Error;

use crate::command::run::RunArgs;
use crate::command::schema::SchemaArgs;
use crate::errors::TooManyErr;
use tracing_log::AsTrace;

mod command;
mod config;
mod errors;
mod types;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct SpmArgs {
    /// Path to the configuration file
    #[arg(short, long)]
    config: Option<PathBuf>,
    /// Profile to use
    #[arg(short, long)]
    profile: Option<String>,
    #[command(subcommand)]
    command: SpmSubcommands,
    #[command(flatten)]
    verbose: Verbosity,
}

#[derive(Debug, Subcommand)]
enum SpmSubcommands {
    /// Run the project
    Run(RunArgs),
    /// Adds a new project
    Add(AddArgs),
    /// Manage profiles
    Profile(ProfileArgs),
    /// Generates config schema definitions
    Schema(SchemaArgs),
}
#[derive(Debug, Args)]
struct ProfileArgs {}

fn main() -> Result<()> {
    let args = SpmArgs::parse();
    tracing_subscriber::fmt()
        .with_max_level(args.verbose.log_level_filter().as_trace())
        .init();

    let mut config = Config::from_path(args.config)?;

    config.select_profile(args.profile)?;

    let result = match args.command {
        SpmSubcommands::Add(add) => add.run(&mut config),
        SpmSubcommands::Run(run) => run.run(&mut config),
        SpmSubcommands::Profile(_) => Ok(()),
        SpmSubcommands::Schema(schema) => schema.run(&mut config),
    };

    if let Err(err) = result {
        match err {
            CommandError::Err(err) => {
                Err(err)?;
                unreachable!()
            }
            CommandError::Canceled => {
                println!("{}", "Operation canceled by user".yellow());
                return Ok(());
            }
        }
    }

    config.save()?;

    Ok(())
}

#[derive(Debug, Error)]
enum CommandError {
    #[error("{}", .0)]
    Err(Report),
    #[error("Operation canceled by user")]
    Canceled,
}

impl From<InquireError> for CommandError {
    fn from(value: InquireError) -> Self {
        match value {
            InquireError::NotTTY => (miette!("The input device is not a TTY")).into(),
            InquireError::InvalidConfiguration(err) => {
                miette!("Bad inquire configuration: {err}").into()
            }
            InquireError::IO(err) => (miette!("IO error: {err}")).into(),
            InquireError::OperationCanceled => Self::Canceled,
            InquireError::OperationInterrupted => Self::Canceled,
            InquireError::Custom(err) => (miette!("{err}")).into(),
        }
    }
}

impl From<Report> for CommandError {
    fn from(value: Report) -> Self {
        Self::Err(value)
    }
}

impl From<TooManyErr> for CommandError {
    fn from(value: TooManyErr) -> Self {
        Self::Err(Report::from(value))
    }
}

/// Helper for wrapping a code block to help with contextualizing errors
/// Better editor support but slightly worse ergonomic than a macro
#[inline(always)]
pub(crate) fn m_try<T>(func: impl FnOnce() -> miette::Result<T>) -> miette::Result<T> {
    func()
}
