use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct InternalConfig {
    pub parser: ParserInternalConfig,
    pub generator: GeneratorInternalConfig,
    pub polisher: PolisherInternalConfig,
}

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct ParserInternalConfig {
    pub rust_input_path: HashMap<Namespace, PathBuf>,
    pub manifest_path: PathBuf,
}

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct GeneratorInternalConfig {
    pub dart: GeneratorDartInternalConfig,
    pub rust: GeneratorRustInternalConfig,
    pub c: GeneratorCInternalConfig,
}

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct GeneratorDartInternalConfig {
    pub dart_output_paths: DartOutputPaths,
    pub dart_enums_style: bool,
    pub class_name: String,
    pub dart_root: Option<String>,
    pub use_bridge_in_method: bool,
    pub wasm_enabled: bool,
    pub dart3: bool,
}

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct DartOutputPaths {
    // TODO details depend on the actual output files...
    pub dart_decl_output_path: HashMap<Namespace, PathBuf>,
    pub dart_impl_output_path: PathBuf,
}

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct GeneratorRustInternalConfig {
    pub rust_crate_dir: PathBuf,
    pub rust_output_path: PathBuf,
    pub inline_rust: bool,
}

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct GeneratorCInternalConfig {
    pub c_output_path: PathBuf,
    pub llvm_path: Vec<PathBuf>,
    pub llvm_compiler_opts: String,
    pub extra_headers: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct PolisherInternalConfig {
    pub duplicated_c_output_path: Vec<PathBuf>,
    pub dart_format_line_length: u32,
    pub add_mod_to_lib: bool,
    pub build_runner: bool,
    pub deps_check: bool,
}

// TODO move?
#[derive(Debug, Clone, serde::Serialize)]
pub struct Namespace(String);
