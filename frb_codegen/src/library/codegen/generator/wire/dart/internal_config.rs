use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct GeneratorWireDartInternalConfig {
    pub use_bridge_in_method: bool,
    pub wasm_enabled: bool,
    pub build_runner: bool,
    pub dart_wire_class_name: String,
    pub llvm_path: Vec<PathBuf>,
    pub llvm_compiler_opts: String,
    pub dart_root: PathBuf,
    pub dart_format_line_length: u32,
}
