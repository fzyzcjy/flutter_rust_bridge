use crate::codegen::config::internal_config::RustInputPathPack;
use crate::codegen::generator::codec::structs::CodecMode;
use crate::codegen::generator::misc::target::TargetOrCommonMap;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct GeneratorWireRustInternalConfig {
    pub rust_input_path_pack: RustInputPathPack,
    pub rust_crate_dir: PathBuf,
    pub web_enabled: bool,
    pub rust_output_path: TargetOrCommonMap<PathBuf>,
    pub c_symbol_prefix: String,
    pub enable_extern_func_and_class: bool,
    pub default_stream_sink_codec: CodecMode,
}
