use crate::codegen::config::config::Config;
use crate::utils::path_utils::path_to_string;
use anyhow::{bail, Context, Error};
use log::debug;
use std::fs;
use std::path::PathBuf;

impl Config {
    pub fn from_files_auto() -> Result<Self, Error> {
        if let Some(config) = Self::from_config_files()? {
            return Ok(config);
        }
        if let Some(config) = Self::from_pubspec_yaml()? {
            return Ok(config);
        }
        bail!("Fail to find any configuration file")
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

    fn from_pubspec_yaml() -> Result<Option<Self>, Error> {
        const PUBSPEC_LOCATION: &str = "pubspec.yaml";

        #[derive(serde::Deserialize)]
        struct Needle {
            #[serde(rename = "flutter_rust_bridge")]
            data: Option<Config>,
        }

        if let Ok(pubspec) = fs::File::open(PUBSPEC_LOCATION) {
            return match serde_yaml::from_reader(pubspec) {
                Ok(Needle { data: Some(data) }) => Ok(Some(data)),
                Ok(Needle { data: None }) => Ok(None),
                Err(err) => Err(Error::new(err).context(format!(
                    "Could not parse the 'flutter_rust_bridge' entry in {PUBSPEC_LOCATION}"
                ))),
            };
        }

        Ok(None)
    }
}

impl Config {
    fn with_base_dir(mut self, base_dir: String) -> Self {
        self.base_dir = Some(base_dir);
        self
    }
}
