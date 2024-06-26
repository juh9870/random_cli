use crate::errors::TooManyErr;
use crate::m_try;
use crossterm::style::Stylize;
use itertools::Itertools;
use miette::{bail, miette, Context, IntoDiagnostic, Result};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;

use schemars::gen::SchemaGenerator;
use schemars::schema::{ObjectValidation, Schema};
use schemars::JsonSchema;
use std::path::PathBuf;

use crate::types::glob::GlobEntry;
use crate::types::ide_runner::IdeRunner;
use crate::types::preprocessors::Preprocessor;
use crate::types::profile::Profile;
use crate::types::project_env::ProjectEnv;
use tracing::{debug, debug_span};
use uncased::Uncased;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "$schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    pub default_profile: Uncased<'static>,
    pub profiles: HashMap<Uncased<'static>, Profile>,
    pub environments: Vec<GlobEntry<ProjectEnv>>,
    pub runners: Vec<GlobEntry<IdeRunner>>,
    pub preprocessors: Vec<Preprocessor>,
    #[serde(skip)]
    current_profile: Option<Uncased<'static>>,
    #[serde(skip)]
    path: Option<PathBuf>,
    #[serde(skip)]
    dirty: bool,
}

impl JsonSchema for Config {
    fn schema_name() -> String {
        "Config".to_string()
    }

    fn schema_id() -> Cow<'static, str> {
        Cow::Borrowed(concat!(module_path!(), "::Config"))
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        {
            let mut schema_object = schemars::schema::SchemaObject {
                instance_type: Some(schemars::schema::InstanceType::Object.into()),
                ..Default::default()
            };
            let object_validation = schema_object.object();

            fn field<T: ?Sized + JsonSchema>(
                gen: &mut SchemaGenerator,
                object_validation: &mut ObjectValidation,
                name: &str,
            ) {
                object_validation
                    .properties
                    .insert(name.to_owned(), gen.subschema_for::<T>());
                object_validation.required.insert(name.to_owned());
            }
            {
                object_validation
                    .properties
                    .insert("$schema".to_owned(), gen.subschema_for::<Option<String>>());
                field::<String>(gen, object_validation, "default_profile");
                field::<HashMap<String, Profile>>(gen, object_validation, "profiles");
                field::<Vec<GlobEntry<ProjectEnv>>>(gen, object_validation, "environments");
                field::<Vec<GlobEntry<IdeRunner>>>(gen, object_validation, "runners");
                field::<Vec<Preprocessor>>(gen, object_validation, "preprocessors");
            }
            Schema::Object(schema_object)
        }
    }
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

                config.dirty = false;
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

    pub fn find_profile(&self, profile: &str) -> Result<Uncased<'static>> {
        let profile_name = {
            let options = self
                .profiles
                .keys()
                .filter(|&p| p.starts_with(profile))
                .cloned()
                .collect_vec();
            match options.len() {
                0 => {
                    bail!("No profiles matching `{}` were found", profile.blue())
                }
                1 => options.into_iter().next().unwrap(),
                _ => {
                    return Err(TooManyErr {
                        query: profile.to_string(),
                        options: options.iter().map(|i| i.to_string()).collect_vec(),
                    }
                    .into());
                }
            }
        };

        Ok(profile_name)
    }

    pub fn select_profile(&mut self, query: Option<&str>) -> Result<()> {
        let profile_name = if let Some(query) = query {
            Uncased::new(self.find_profile(query)?.to_string())
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
                    .as_deref()
                    .expect("Current profile should be set before using config"),
            )
            .expect("`current_profile` should point at a correct profile")
    }

    pub fn get_profile_mut(&mut self) -> &mut Profile {
        self.dirty = true;
        self.profiles
            .get_mut(
                self.current_profile
                    .as_deref()
                    .expect("Current profile should be set before using config"),
            )
            .expect("`current_profile` should point at a correct profile")
    }

    pub fn current_profile_name(&self) -> &Uncased<'static> {
        self.current_profile
            .as_ref()
            .expect("Current profile should be set before using config")
    }

    pub fn set_dirty(&mut self) {
        self.dirty = true;
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
        serde_json::from_str(include_str!("./default_config.json"))
            .expect("Failed to deserialize default configuration")
    }
}
