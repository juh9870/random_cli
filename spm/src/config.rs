use crate::errors::TooManyErr;
use crate::m_try;
use crate::profile::Profile;
use crossterm::style::Stylize;
use itertools::Itertools;
use miette::{bail, miette, Context, IntoDiagnostic, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use std::path::PathBuf;

use tracing::{debug, debug_span};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    default_profile: String,
    profiles: HashMap<String, Profile>,
    #[serde(skip)]
    current_profile: Option<String>,
    #[serde(skip)]
    path: Option<PathBuf>,
    #[serde(skip)]
    dirty: bool,
}

impl Config {
    pub fn from_path(config_path: Option<PathBuf>) -> Result<Config> {
        let config_path = if let Some(path) = config_path {
            path
        } else {
            let home_dir = home::home_dir().ok_or_else(||miette!("Failed to get user home directory, please specify configuration file location manually"))?;
            home_dir.join(".spm.v2.json")
        };
        let mut config = if config_path.exists() {
            m_try(|| {
                let config_content = fs_err::read_to_string(&config_path)
                    .into_diagnostic()
                    .context("Failed to read configuration file")?;
                let mut config: Config = serde_json::de::from_str(&config_content)
                    .into_diagnostic()
                    .context("Failed to deserialize configuration file content")?;

                config.dirty = true;
                Ok(config)
            })
            .with_context(|| {
                format!("Failed to get configuration from {}", config_path.display())
            })?
        } else {
            Default::default()
        };

        config.path = Some(config_path);

        Ok(config)
    }

    pub fn select_profile(&mut self, query: Option<String>) -> Result<()> {
        let profile_name = if let Some(profile) = query {
            let options = self
                .profiles
                .keys()
                .filter(|&p| p.starts_with(&profile))
                .cloned()
                .collect_vec();
            match options.len() {
                0 => {
                    bail!("No profiles matching `{}` were found", profile.blue())
                }
                1 => options.into_iter().next().unwrap(),
                _ => {
                    return Err(TooManyErr {
                        query: profile,
                        options,
                    }
                    .into());
                }
            }
        } else {
            self.default_profile.clone()
        };
        self.current_profile = Some(profile_name);

        Ok(())
    }

    pub fn get_profile(&self) -> &Profile {
        self.profiles
            .get(
                self.current_profile
                    .as_ref()
                    .expect("Current profile should be set before using config"),
            )
            .expect("`current_profile` should point at a correct profile")
    }

    pub fn get_profile_mut(&mut self) -> &mut Profile {
        self.dirty = true;
        self.profiles
            .get_mut(
                self.current_profile
                    .as_ref()
                    .expect("Current profile should be set before using config"),
            )
            .expect("`current_profile` should point at a correct profile")
    }

    /// Saves the configuration, if dirty
    pub fn save(&mut self) -> Result<()> {
        let _guard = debug_span!("Saving config").entered();
        if !self.dirty {
            debug!("Config is not dirty, saving aborted");
            return Ok(());
        }

        self.force_save()
    }

    /// Saves the configuration
    pub fn force_save(&mut self) -> Result<()> {
        fs_err::write(
            self.path.as_ref().unwrap(),
            serde_json::ser::to_string_pretty(self)
                .into_diagnostic()
                .context("Failed to serialize configuration")?,
        )
        .into_diagnostic()
        .context("Failed to write configuration file")?;

        debug!(
            "Configuration saved to {}",
            self.path.as_ref().unwrap().display()
        );

        self.dirty = false;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_profile: "default".to_string(),
            profiles: {
                let mut profiles = HashMap::new();
                profiles.insert(
                    "default".to_string(),
                    Profile {
                        name: "default".to_string(),
                        projects: vec![],
                    },
                );
                profiles
            },
            current_profile: None,
            path: None,
            dirty: true,
        }
    }
}
