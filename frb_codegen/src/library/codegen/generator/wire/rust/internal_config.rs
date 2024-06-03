use crate::codegen::generator::codec::structs::CodecMode;
use crate::codegen::generator::misc::target::TargetOrCommonMap;
use crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct GeneratorWireRustInternalConfig {
    pub rust_crate_dir: PathBuf,
    pub web_enabled: bool,
    pub rust_output_path: TargetOrCommonMap<PathBuf>,
    pub c_symbol_prefix: String,
    pub has_ffigen: bool,
    pub default_stream_sink_codec: CodecMode,
    pub default_rust_opaque_codec: RustOpaqueCodecMode,
}
