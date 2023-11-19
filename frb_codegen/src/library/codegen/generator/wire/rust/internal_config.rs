use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct GeneratorWireRustInternalConfig {
    pub rust_wire_mod: String,
    pub wasm_enabled: bool,
    pub rust_common_output_path: String,
    pub rust_io_output_path: String,
    pub rust_wasm_output_path: String,
}
