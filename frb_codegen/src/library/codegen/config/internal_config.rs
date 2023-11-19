use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::parser::internal_config::ParserInternalConfig;
use crate::codegen::preparer::PreparerInternalConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct InternalConfig {
    pub preparer: PreparerInternalConfig,
    pub parser: ParserInternalConfig,
    pub generator: GeneratorInternalConfig,
    pub polisher: PolisherInternalConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct GeneratorInternalConfig {
    pub api_dart: GeneratorApiDartInternalConfig,
    pub wire: GeneratorWireInternalConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct GeneratorWireInternalConfig {
    pub dart: GeneratorDartWireInternalConfig,
    pub rust: GeneratorRustWireInternalConfig,
    pub c: GeneratorCWireInternalConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct GeneratorDartWireInternalConfig {
    pub dart_impl_output_path: PathBuf,
    pub dart_enums_style: bool,
    pub dart_class_name: HashMap<Namespace, String>,
    pub dart_root: PathBuf,
    pub use_bridge_in_method: bool,
    pub wasm_enabled: bool,
    pub dart3: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct GeneratorRustWireInternalConfig {
    pub rust_crate_dir: PathBuf,
    pub rust_output_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct GeneratorCWireInternalConfig {
    pub c_output_path: PathBuf,
    pub llvm_path: Vec<PathBuf>,
    pub llvm_compiler_opts: String,
    pub extra_headers: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct PolisherInternalConfig {
    pub duplicated_c_output_path: Vec<PathBuf>,
    pub dart_format_line_length: u32,
    pub add_mod_to_lib: bool,
    pub build_runner: bool,
}

// TODO move?
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct Namespace {
    pub name: String,
}

impl From<String> for Namespace {
    fn from(name: String) -> Self {
        Self { name }
    }
}
