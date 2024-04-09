use crate::codegen::config::internal_config::RustInputPathPack;
use crate::codegen::generator::codec::structs::{CodecMode, CodecModePack};
use crate::codegen::ir::ty::rust_opaque::RustOpaqueCodecMode;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ParserInternalConfig {
    pub rust_input_path_pack: RustInputPathPack,
    pub rust_crate_dir: PathBuf,
    pub force_codec_mode_pack: Option<CodecModePack>,
    pub default_stream_sink_codec: CodecMode,
    pub default_rust_opaque_codec: RustOpaqueCodecMode,
}
