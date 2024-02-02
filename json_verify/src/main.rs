use clap::Parser;
use color_eyre::eyre;
use color_eyre::owo_colors::OwoColorize;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use tracing::error_span;
use walkdir::WalkDir;

/// Validates all JSON files in a directory
#[derive(Parser, Debug)]
struct Args {
    /// Directory to validate files in. Defaults to current directory if not provided
    dir: Option<PathBuf>,
    /// Determines whenever files with failed reads are included into error output or not
    #[arg(long)]
    skip_failed_reads: bool,
}

fn validate_file(path: &Path, errors: &mut HashMap<String, String>) -> color_eyre::Result<()> {
    let _guard = error_span!(
        "Reading file",
        path = %path.display()
    )
    .entered();

    let data = fs_err::read(path)?;

    let data = match String::from_utf8(data) {
        Ok(data) => data,
        Err(err) => {
            errors.insert(
                path.to_string_lossy().to_string(),
                format!("File is not a valid UTF8: {}", err),
            );
            return Ok(());
        }
    };

    match serde_json::from_str::<serde_json::Value>(&data) {
        Ok(data) => data,
        Err(err) => {
            errors.insert(
                path.to_string_lossy().to_string(),
                format!("File is not a valid JSON: {}", err),
            );
            return Ok(());
        }
    };
    Ok(())
}

fn main() -> color_eyre::Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;

    let Args {
        dir,
        skip_failed_reads: skip_failed_files,
    } = Args::parse();
    let dir = match dir {
        None => env::current_dir()?,
        Some(dir) => dir,
    };

    let mut report_errors = HashMap::new();

    let pb = ProgressBar::new_spinner();

    pb.set_style(
        ProgressStyle::with_template("{prefix} {spinner} {wide_msg}")
            .unwrap()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "),
    );

    for (i, entry) in WalkDir::new(dir).into_iter().enumerate() {
        match entry {
            Ok(data) => {
                let err_len = report_errors.len();
                let count = if err_len > 0 {
                    err_len.bold().red().to_string()
                } else {
                    err_len.bold().green().to_string()
                };
                pb.set_prefix(format!("[{}/{}]", count, (i + 1).bold().dimmed()));
                pb.set_message(data.path().to_string_lossy().to_string());
                pb.inc(1);
                if !data.file_type().is_file() {
                    continue;
                }

                let Some(extension) = data.path().extension().map(|e| e.to_ascii_lowercase())
                else {
                    continue;
                };

                if extension != "json" {
                    continue;
                }

                validate_file(data.path(), &mut report_errors)?;
            }
            Err(err) => {
                if skip_failed_files {
                    continue;
                }
                let path = err
                    .path()
                    .map(|e| e.to_string_lossy().to_string())
                    .unwrap_or_else(|| "<unknown>".to_string());
                let err = format!("Failed to read the file at {path}");
                report_errors.insert(path, err);
            }
        }
    }

    pb.finish_and_clear();

    if report_errors.len() > 0 {
        let len = report_errors.len();
        let report = report_errors
            .into_iter()
            .map(|(k, v)| format!("{}: {}", k.bold(), v.red()))
            .join("\n");
        println!(
            "{} {}\n\n{}",
            len.bold().red(),
            "broken files detected".red(),
            report
        )
    } else {
        pb.println("All files are valid".green().to_string())
    }

    Ok(())
}
