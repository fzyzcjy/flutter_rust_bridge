//! Code generator for `flutter_rust_bridge`

mod config_parser;
mod internal_config;
mod internal_config_parser;

pub use config_parser::*;

use clap::ValueEnum;
use log::debug;
use serde::Deserialize;

/// Execute the main code generator
pub fn generate(config: &Config) -> anyhow::Result<()> {
    debug!("config={config:?}");

    todo!("generate")
}

/// Configuration for code generation
/// Refer to `GenerateCommandArgs` for documentations
#[derive(Debug, Deserialize)]
pub struct Config {
    pub rust_input: Option<Vec<String>>,
    pub dart_output: Option<String>,
    pub dart_decl_output: Option<String>,
    pub c_output: Option<Vec<String>>,
    pub extra_c_output: Option<Vec<String>>,
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
    pub wasm: Option<bool>,
    pub inline_rust: Option<bool>,
    pub deps_check: Option<bool>,
    pub dart3: Option<bool>,
    pub keep_going: Option<bool>,
    pub dump: Option<Vec<ConfigDump>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, ValueEnum, enum_iterator::Sequence)]
#[serde(rename_all = "snake_case")]
pub enum ConfigDump {
    Config,
    Ir,
}

