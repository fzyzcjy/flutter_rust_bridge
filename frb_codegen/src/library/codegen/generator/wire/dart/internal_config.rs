use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct GeneratorWireDartInternalConfig {
    pub use_bridge_in_method: bool,
    pub wasm_enabled: bool,
    pub dart_wire_class_name: String,
    pub llvm_path: Vec<PathBuf>,
    pub llvm_compiler_opts: String,
    pub dart_root: PathBuf,
    pub extra_headers: String,
    pub dart_impl_output_path: PathBuf,
}
