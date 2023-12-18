use crate::codegen::config::internal_config::RustInputPathPack;
use crate::codegen::generator::misc::target::TargetOrCommonMap;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct GeneratorWireRustInternalConfig {
    pub rust_input_path_pack: RustInputPathPack,
    pub rust_crate_dir: PathBuf,
    pub web_enabled: bool,
    pub rust_output_path: TargetOrCommonMap<PathBuf>,
}
