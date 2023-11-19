use crate::codegen::generator::misc::TargetOrCommonMap;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct GeneratorWireRustInternalConfig {
    pub rust_crate_dir: PathBuf,
    pub rust_wire_mod: String,
    pub wasm_enabled: bool,
    pub rust_output_path: TargetOrCommonMap<PathBuf>,
}
