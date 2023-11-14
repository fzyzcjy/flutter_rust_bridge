use std::fs;
use anyhow::{bail, Context, Error};
use log::debug;
use crate::codegen::Config;

impl Config {
    pub fn from_files() -> Result<Self, Error> {
        if let Some(config) = Self::from_config_files()? {
            return Ok(config);
        }
        if let Some(config) = Self::from_pubspec_yaml()? {
            return Ok(config);
        }
        bail!("todo") // TODO
    }

    fn from_config_files() -> Result<Option<Self>, Error> {
        const CONFIG_LOCATIONS: [&str; 3] = [
            ".flutter_rust_bridge.yml",
            ".flutter_rust_bridge.yaml",
            ".flutter_rust_bridge.json",
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
            return serde_yaml::from_reader(file)
                .with_context(|| format!("Could not parse {location}"));
        }

        Ok(None)
    }

    fn from_pubspec_yaml() -> Result<Option<Self>, Error> {
        const PUBSPEC_LOCATION: &str= "pubspec.yaml";

        #[derive(serde::Deserialize)]
        struct Needle {
            #[serde(rename = "flutter_rust_bridge")]
            data: Option<Config>,
        }

        let mut hint = "fill in .flutter_rust_bridge.yml with your config.".to_owned();
        if let Ok(pubspec) = fs::File::open(PUBSPEC_LOCATION) {
            match serde_yaml::from_reader(pubspec) {
                Ok(Needle { data: Some(data) }) => return Ok(Some(data)),
                Ok(Needle { data: None }) => {
                    hint = format!("create an entry called 'flutter_rust_bridge' in {location} with your config.");
                }
                Err(err) => {
                    return Err(Error::new(err).context(format!(
                        "Could not parse the 'flutter_rust_bridge' entry in {location}"
                    )));
                }
            }
        }

        Ok(None)
    }
}
