use std::fs;
use anyhow::Context;
use clap::Parser;
use crate::config::raw_opts::RawOpts;

impl RawOpts {
    /// Parses options from arguments, or from a config file if no arguments are given.
    /// Terminates the program if argument validation fails.
    #[cfg(feature = "serde")]
    pub fn try_parse_args_or_yaml() -> anyhow::Result<Self> {
        const CONFIG_LOCATIONS: [&str; 3] = [
            ".flutter_rust_bridge.yml",
            ".flutter_rust_bridge.yaml",
            ".flutter_rust_bridge.json",
        ];
        const PUBSPEC_LOCATIONS: [&str; 2] = ["pubspec.yaml", "pubspec.yml"];

        let has_args = std::env::args_os().len() > 1;
        match Self::try_parse() {
            Ok(opts) => Ok(opts),
            Err(err) if has_args => err.exit(),
            // Try to parse a command file, if exists
            Err(err) => {
                for location in CONFIG_LOCATIONS {
                    if let Ok(file) = fs::File::open(location) {
                        eprintln!("Found config file {location}");
                        return serde_yaml::from_reader(file)
                            .with_context(|| format!("Could not parse {location}"));
                    }
                }

                let mut hint = "fill in .flutter_rust_bridge.yml with your config.".to_owned();
                for location in PUBSPEC_LOCATIONS {
                    if let Ok(pubspec) = fs::File::open(location) {
                        #[derive(serde::Deserialize)]
                        struct Needle {
                            #[serde(rename = "flutter_rust_bridge")]
                            data: Option<RawOpts>,
                        }
                        match serde_yaml::from_reader(pubspec) {
                            Ok(Needle { data: Some(data) }) => return Ok(data),
                            Ok(Needle { data: None }) => {
                                hint = format!("create an entry called 'flutter_rust_bridge' in {location} with your config.");
                            }
                            Err(err) => {
                                return Err(anyhow::Error::new(err).context(format!(
                                    "Could not parse the 'flutter_rust_bridge' entry in {location}"
                                )));
                            }
                        }
                    }
                }

                _ = err.print();
                eprintln!("Hint: To call flutter_rust_bridge_codegen with no arguments, {hint}");
                std::process::exit(1)
            }
        }
    }
}

