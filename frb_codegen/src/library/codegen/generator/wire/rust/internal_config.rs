use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::{TargetOrCommon, TargetOrCommonMap};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct GeneratorWireRustInternalConfig {
    pub rust_crate_dir: PathBuf,
    pub rust_wire_mod: String,
    pub wasm_enabled: bool,
    pub add_mod_to_lib: bool,
    pub rust_output_path: TargetOrCommonMap<PathBuf>,
}
