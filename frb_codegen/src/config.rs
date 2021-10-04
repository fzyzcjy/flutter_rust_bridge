use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub rust: ConfigRust,
    pub dart: ConfigDart,
    pub c: ConfigC,
}

impl Config {
    pub fn read(config_path: &str) -> Self {
        let raw_config: Config =
            serde_yaml::from_str(&fs::read_to_string(config_path).unwrap()).unwrap();
        raw_config.canonicalize(PathBuf::from(config_path).parent().unwrap())
    }

    pub fn canonicalize(self, base_dir: &Path) -> Config {
        let canon_dir = |sub_path: &str| {
            let mut path = PathBuf::from(base_dir);
            path.push(sub_path);
            path.into_os_string().into_string().unwrap()
        };

        Config {
            rust: ConfigRust {
                crate_dir: canon_dir(&self.rust.crate_dir),
                api_path: canon_dir(&self.rust.api_path),
                wire_path: canon_dir(&self.rust.wire_path),
            },
            dart: ConfigDart {
                api_class_name: self.dart.api_class_name,
                wire_class_name: self.dart.wire_class_name,
                api_path: canon_dir(&self.dart.api_path),
                wire_path: canon_dir(&self.dart.wire_path),
                format_line_length: self.dart.format_line_length,
            },
            c: ConfigC {
                wire_path: canon_dir(&self.c.wire_path),
            },
        }
    }
}

/// "wire" := the things that are really passed through via C ABI (this name is similar to how Protobuf name things)
/// "api" := the interface that you, as a user, should use.
#[derive(Debug, PartialEq, Deserialize)]
pub struct ConfigRust {
    pub crate_dir: String,
    pub api_path: String,
    pub wire_path: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ConfigDart {
    pub api_class_name: String,
    pub wire_class_name: String,
    pub api_path: String,
    pub wire_path: String,
    #[serde(default = "default_format_line_length")]
    pub format_line_length: i32,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ConfigC {
    pub wire_path: String,
}

fn default_format_line_length() -> i32 {
    80
}
