use serde::Deserialize;

/// Clarification for "api" and "wire" used in this project:
/// "api" := The interface that you, as a user, should use. You have defined a Rust APi, and will
///          use the generated Dart api class in your Dart/Flutter code.
/// "wire" := The things that you should not care about (only give it a place to live in is enough).
/// They are the underlying things that are really passed through via C ABI
#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub rust: ConfigRust,
    pub dart: ConfigDart,
    pub c: ConfigC,
}

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
