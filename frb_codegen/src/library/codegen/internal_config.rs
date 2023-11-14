use std::path::PathBuf;

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct InternalConfig {
    pub parser: ParserInternalConfig,
    pub generator: GeneratorInternalConfig,
    pub polisher: PolisherInternalConfig,
}

pub(crate) struct ParserInternalConfig {
    pub rust_input_path: Vec<PathBuf>,
    pub manifest_path: PathBuf,
}

pub(crate) struct GeneratorInternalConfig {
    pub dart: GeneratorDartInternalConfig,
    pub rust: GeneratorRustInternalConfig,
    pub c: GeneratorCInternalConfig,
}

pub(crate) struct GeneratorDartInternalConfig {
    pub dart_output_path: PathBuf,
    pub dart_decl_output_path: Option<PathBuf>,
    pub dart_enums_style: bool,
    pub class_name: String,
    pub dart_root: Option<String>,
    pub use_bridge_in_method: bool,
    pub wasm_enabled: bool,
    pub dart3: bool,
}

pub(crate) struct GeneratorRustInternalConfig {
    pub rust_crate_dir: String,
    pub rust_output_path: PathBuf,
    pub inline_rust: bool,
}

pub(crate) struct GeneratorCInternalConfig {
    pub c_output_path: Vec<PathBuf>,
    pub llvm_path: Vec<PathBuf>,
    pub llvm_compiler_opts: String,
    pub extra_headers: String,
}

// TODO naming
pub(crate) struct PolisherInternalConfig {
    pub extra_c_output_path: Vec<PathBuf>,
    pub dart_format_line_length: u32,
    pub add_mod_to_lib: bool,
    pub build_runner: bool,
    pub deps_check: bool,
}
