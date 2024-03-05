use crate::codegen::config::config::Config;
use crate::utils::path_utils::path_to_string;
use anyhow::{bail, Context, Error};
use log::debug;
use std::fs;
use std::path::PathBuf;

impl Config {
    pub fn from_files_auto() -> Result<Self, Error> {
        const PUBSPEC_LOCATION: &str = "pubspec.yaml";

        if let Some(config) = Self::from_config_files()? {
            return Ok(config);
        }
        if let Some(config) = Self::from_pubspec_yaml(PUBSPEC_LOCATION)? {
            return Ok(config);
            // This will stop the whole generator and tell the users, so we do not care about testing it
            // frb-coverage:ignore-start
        }
        bail!("Fail to find any configuration file")
        // frb-coverage:ignore-end
    }

    fn from_config_files() -> Result<Option<Self>, Error> {
        const CONFIG_LOCATIONS: [&str; 6] = [
            ".flutter_rust_bridge.yml",
            ".flutter_rust_bridge.yaml",
            ".flutter_rust_bridge.json",
            "flutter_rust_bridge.yml",
            "flutter_rust_bridge.yaml",
            "flutter_rust_bridge.json",
        ];

        for location in CONFIG_LOCATIONS {
            if let Some(config) = Self::from_config_file(location)? {
                return Ok(Some(config));
            }
        }

        Ok(None)
    }

    pub fn from_config_file(location: &str) -> Result<Option<Self>, Error> {
        if let Ok(file) = fs::File::open(location) {
            debug!("Found config file {location}");
            let raw: Config = serde_yaml::from_reader(file)
                .with_context(|| format!("Could not parse {location}"))?;
            let base_dir = path_to_string(PathBuf::from(location).parent().context("no parent")?)?;
            return Ok(Some(raw.with_base_dir(base_dir)));
        }

        Ok(None)
    }

    /// Loads the [`Config`] from a specified `pubspec.yaml` file.
    ///
    /// Returns [`None`] if it doesn't contain the `flutter_rust_bridge` section somewhere in the file.
    pub fn from_pubspec_yaml(location: &str) -> Result<Option<Self>, Error> {
        #[derive(serde::Deserialize)]
        struct Needle {
            #[serde(rename = "flutter_rust_bridge")]
            data: Option<Config>,
        }

        if let Ok(pubspec) = fs::File::open(location) {
            return match serde_yaml::from_reader(pubspec) {
                Ok(Needle { data: Some(data) }) => Ok(Some(data)),
                // This will stop the whole generator and tell the users, so we do not care about testing it
                // frb-coverage:ignore-start
                Ok(Needle { data: None }) => Ok(None),
                Err(err) => Err(Error::new(err).context(format!(
                    "Could not parse the 'flutter_rust_bridge' entry in {location}"
                ))),
            };
        }

        Ok(None)
        // frb-coverage:ignore-end
    }
}

impl Config {
    fn with_base_dir(mut self, base_dir: String) -> Self {
        self.base_dir = Some(base_dir);
        self
    }
}
