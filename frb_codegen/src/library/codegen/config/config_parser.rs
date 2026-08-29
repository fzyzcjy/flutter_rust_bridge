use crate::codegen::config::config::Config;
use crate::utils::path_utils::path_to_string;
use anyhow::{Context, Error};
use log::debug;
use std::fs;
use std::path::PathBuf;

impl Config {
    pub fn from_files_auto() -> anyhow::Result<Self> {
        Self::from_files_auto_option()?.context("Fail to find any configuration file")
    }

    // Only used internally
    #[doc(hidden)]
    pub fn from_files_auto_option() -> anyhow::Result<Option<Self>> {
        const PUBSPEC_LOCATION: &str = "pubspec.yaml";

        if let Some(config) = Self::from_config_files()? {
            return Ok(Some(config));
        }
        if let Some(config) = Self::from_pubspec_yaml(PUBSPEC_LOCATION)? {
            return Ok(Some(config));
            // This will stop the whole generator and tell the users, so we do not care about testing it
            // frb-coverage:ignore-start
        }
        Ok(None)
        // frb-coverage:ignore-end
    }

    fn from_config_files() -> anyhow::Result<Option<Self>> {
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

    pub fn from_config_file(location: &str) -> anyhow::Result<Option<Self>> {
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
    pub fn from_pubspec_yaml(location: &str) -> anyhow::Result<Option<Self>> {
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

#[cfg(test)]
mod tests {
    use super::Config;
    use crate::utils::test_utils::CurrentDirGuard;
    use serial_test::serial;
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;

    fn write_file(root: &Path, relative_path: &str, content: &str) -> anyhow::Result<()> {
        let path = root.join(relative_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, content)?;
        Ok(())
    }

    /// Discovers each supported automatic config filename.
    #[test]
    #[serial]
    fn discovers_config_files_in_the_documented_order() -> anyhow::Result<()> {
        const LOCATIONS: [&str; 6] = [
            ".flutter_rust_bridge.yml",
            ".flutter_rust_bridge.yaml",
            ".flutter_rust_bridge.json",
            "flutter_rust_bridge.yml",
            "flutter_rust_bridge.yaml",
            "flutter_rust_bridge.json",
        ];
        let temp_dir = TempDir::new()?;
        let _guard = CurrentDirGuard::change_to(temp_dir.path())?;

        for (index, location) in LOCATIONS.iter().enumerate() {
            write_file(
                temp_dir.path(),
                location,
                &format!("rust_input: crate::location_{index}\n"),
            )?;
            let config = Config::from_files_auto_option()?.expect("config should be found");
            assert_eq!(config.rust_input, Some(format!("crate::location_{index}")));
            fs::remove_file(temp_dir.path().join(location))?;
        }

        Ok(())
    }

    /// Chooses the earliest config file before later sources.
    #[test]
    #[serial]
    fn prefers_the_first_config_file_over_later_files_and_pubspec() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        write_file(
            temp_dir.path(),
            "flutter_rust_bridge.json",
            r#"{"rust_input":"crate::last"}"#,
        )?;
        write_file(
            temp_dir.path(),
            ".flutter_rust_bridge.yaml",
            "rust_input: crate::first\n",
        )?;
        write_file(
            temp_dir.path(),
            "pubspec.yaml",
            "flutter_rust_bridge:\n  rust_input: crate::pubspec\n",
        )?;
        let _guard = CurrentDirGuard::change_to(temp_dir.path())?;

        let config = Config::from_files_auto_option()?.expect("config should be found");

        assert_eq!(config.rust_input.as_deref(), Some("crate::first"));
        Ok(())
    }

    /// Parses YAML and JSON config files with their parent directory.
    #[test]
    fn parses_yaml_and_json_config_files_and_sets_the_parent_as_base_dir() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        write_file(temp_dir.path(), "nested/config.yaml", "dart3: false\n")?;
        write_file(temp_dir.path(), "nested/config.json", r#"{"dart3":true}"#)?;

        let yaml = Config::from_config_file(
            &temp_dir.path().join("nested/config.yaml").to_string_lossy(),
        )?
        .expect("YAML config should be found");
        let json = Config::from_config_file(
            &temp_dir.path().join("nested/config.json").to_string_lossy(),
        )?
        .expect("JSON config should be found");

        assert_eq!(yaml.dart3, Some(false));
        assert_eq!(json.dart3, Some(true));
        assert_eq!(
            yaml.base_dir,
            Some(
                temp_dir
                    .path()
                    .join("nested")
                    .to_string_lossy()
                    .into_owned()
            )
        );
        assert_eq!(
            json.base_dir,
            Some(
                temp_dir
                    .path()
                    .join("nested")
                    .to_string_lossy()
                    .into_owned()
            )
        );
        Ok(())
    }

    /// Returns no config when the requested file is absent.
    #[test]
    fn returns_none_for_missing_config_files() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;

        assert!(
            Config::from_config_file(&temp_dir.path().join("missing.yaml").to_string_lossy())?
                .is_none()
        );
        Ok(())
    }

    /// Adds file-path context to malformed config-file errors.
    #[test]
    fn reports_config_file_context_for_unknown_fields() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let config_path = temp_dir.path().join("nested/config.yaml");
        write_file(
            temp_dir.path(),
            "nested/config.yaml",
            "misspelled_dart3: false\n",
        )?;

        let error = Config::from_config_file(&config_path.to_string_lossy()).unwrap_err();
        let config_path = config_path.to_string_lossy();

        assert!(error.to_string().contains("Could not parse"));
        assert!(error.to_string().contains(config_path.as_ref()));
        assert!(error
            .chain()
            .any(|cause| cause.to_string().contains("misspelled_dart3")));
        Ok(())
    }

    /// Parses the nested flutter_rust_bridge pubspec section.
    #[test]
    fn parses_nested_pubspec_configuration() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let pubspec = temp_dir.path().join("pubspec.yaml");
        write_file(
            temp_dir.path(),
            "pubspec.yaml",
            "name: example\nflutter_rust_bridge:\n  rust_input: crate::api\n  dart3: false\n",
        )?;

        let config = Config::from_pubspec_yaml(&pubspec.to_string_lossy())?
            .expect("nested configuration should be found");

        assert_eq!(config.rust_input.as_deref(), Some("crate::api"));
        assert_eq!(config.dart3, Some(false));
        assert_eq!(config.base_dir, None);
        Ok(())
    }

    /// Returns no config for a pubspec without the section.
    #[test]
    fn returns_none_when_pubspec_has_no_configuration_section() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let pubspec = temp_dir.path().join("pubspec.yaml");
        write_file(temp_dir.path(), "pubspec.yaml", "name: example\n")?;

        assert!(Config::from_pubspec_yaml(&pubspec.to_string_lossy())?.is_none());
        Ok(())
    }

    /// Adds pubspec context to malformed nested configuration errors.
    #[test]
    fn reports_pubspec_context_for_malformed_configuration() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let pubspec = temp_dir.path().join("pubspec.yaml");
        write_file(
            temp_dir.path(),
            "pubspec.yaml",
            "flutter_rust_bridge:\n  misspelled_dart3: false\n",
        )?;

        let error = Config::from_pubspec_yaml(&pubspec.to_string_lossy()).unwrap_err();

        assert!(error.to_string().contains("flutter_rust_bridge"));
        assert!(error
            .chain()
            .any(|cause| cause.to_string().contains("misspelled_dart3")));
        Ok(())
    }

    /// Returns no automatic config when every source is absent.
    #[test]
    #[serial]
    fn returns_none_when_no_automatic_config_file_exists() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let _guard = CurrentDirGuard::change_to(temp_dir.path())?;

        assert!(Config::from_files_auto_option()?.is_none());
        Ok(())
    }

    /// Reports the public missing-source error when no config exists.
    #[test]
    #[serial]
    fn reports_an_error_when_no_automatic_config_file_exists() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let _guard = CurrentDirGuard::change_to(temp_dir.path())?;

        let error = Config::from_files_auto().unwrap_err();

        assert!(error
            .to_string()
            .contains("Fail to find any configuration file"));
        Ok(())
    }
}
