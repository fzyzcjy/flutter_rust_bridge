use crate::codegen::generator::misc::TargetOrCommon;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct GeneratorWireRustInternalConfig {
    pub rust_wire_mod: String,
    pub wasm_enabled: bool,
    pub rust_output_path: HashMap<TargetOrCommon, PathBuf>,
}
