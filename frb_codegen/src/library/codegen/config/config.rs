use clap::ValueEnum;
use serde::Deserialize;

/// Configuration for code generation
/// Refer to `GenerateCommandArgs` for documentations
#[derive(Debug, Deserialize)]
pub struct Config {
    pub base_dir: Option<String>,
    pub rust_input: String,
    pub dart_output: String,
    pub c_output: String,
    pub duplicated_c_output: Option<Vec<String>>,
    pub rust_crate_dir: Option<String>,
    pub rust_output: Option<String>,
    pub dart_class_name: Option<String>,
    pub dart_format_line_length: Option<u32>,
    pub dart_enums_style: Option<bool>,
    pub add_mod_to_lib: Option<bool>,
    pub llvm_path: Option<Vec<String>>,
    pub llvm_compiler_opts: Option<String>,
    pub dart_root: Option<String>,
    pub build_runner: Option<bool>,
    pub use_bridge_in_method: Option<bool>,
    pub extra_headers: Option<String>,
    pub wasm: Option<bool>,
    pub deps_check: Option<bool>,
    pub dart3: Option<bool>,
    // TODO handle this
    pub dump: Option<Vec<ConfigDump>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, ValueEnum, enum_iterator::Sequence)]
#[serde(rename_all = "snake_case")]
pub enum ConfigDump {
    // TODO can dump more
    Config,
    Ir,
}
