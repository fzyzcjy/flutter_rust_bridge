use crate::codegen::generator::misc::target::TargetOrCommonMap;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct GeneratorWireDartInternalConfig {
    pub(crate) has_ffigen: bool,
    pub web_enabled: bool,
    pub llvm_path: Vec<PathBuf>,
    pub llvm_compiler_opts: String,
    pub dart_root: PathBuf,
    pub extra_headers: String,
    pub dart_impl_output_path: TargetOrCommonMap<PathBuf>,
    pub dart_output_class_name_pack: DartOutputClassNamePack,
    pub default_external_library_loader:
        GeneratorWireDartDefaultExternalLibraryLoaderInternalConfig,
    pub(crate) c_symbol_prefix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct GeneratorWireDartDefaultExternalLibraryLoaderInternalConfig {
    pub stem: String,
    pub io_directory: String,
    pub web_prefix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct DartOutputClassNamePack {
    pub entrypoint_class_name: String,
    pub api_class_name: String,
    pub api_impl_class_name: String,
    pub api_impl_platform_class_name: String,
    pub wire_class_name: String,
    pub wasm_module_name: String,
}
