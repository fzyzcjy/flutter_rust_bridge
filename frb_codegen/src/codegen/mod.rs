//! Code generator for `flutter_rust_bridge`

use clap::ValueEnum;
use serde::Deserialize;

/// Execute the main code generator
pub fn generate(config: &Config) -> anyhow::Result<()> {
    todo!("generate")
}

/// Configuration for code generation
/// Refer to `GenerateCommandArgs` for documentations
#[derive(Debug, Deserialize)]
pub struct Config {
    pub rust_input: Option<String>,
    pub dart_output: Option<String>,
    pub config_file: Option<String>,
    pub dart_decl_output: Option<String>,
    pub c_output: Option<String>,
    pub rust_crate_dir: Option<String>,
    pub rust_output: Option<String>,
    pub class_name: Option<String>,
    pub dart_format_line_length: Option<u32>,
    pub dart_enums_style: Option<bool>,
    pub add_mod_to_lib: Option<bool>,
    pub llvm_path: Option<Vec<String>>,
    pub llvm_compiler_opts: Option<String>,
    pub dart_root: Option<String>,
    pub build_runner: Option<bool>,
    pub use_bridge_in_method: Option<bool>,
    pub extra_headers: Option<String>,
    pub verbose: Option<bool>,
    pub wasm: Option<bool>,
    pub inline_rust: Option<bool>,
    pub skip_deps_check: Option<bool>,
    pub dump: Option<Vec<ConfigDump>>,
    pub dart3: Option<bool>,
    pub keep_going: Option<bool>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, ValueEnum, enum_iterator::Sequence)]
#[serde(rename_all = "snake_case")]
pub enum ConfigDump {
    Config,
    Ir,
}

