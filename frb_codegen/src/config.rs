use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub rust: ConfigRust,
    pub dart: ConfigDart,
    pub c: ConfigC,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ConfigRust {
    pub crate_dir: String,
    pub input_path: String,
    pub output_path: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ConfigDart {
    pub output_path: String,
    pub output_class_name: String,
    #[serde(default = "default_format_line_length")]
    pub format_line_length: i32,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ConfigC {
    pub output_path: String,
}

fn default_format_line_length() -> i32 {
    80
}
