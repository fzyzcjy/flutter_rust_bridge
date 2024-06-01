use crate::codegen::generator::codec::structs::{CodecMode, CodecModePack};
use crate::codegen::ir::mir::namespace::Namespace;
use crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ParserInternalConfig {
    pub rust_input_namespace_pack: RustInputNamespacePack,
    pub rust_crate_dir: PathBuf,
    pub force_codec_mode_pack: Option<CodecModePack>,
    pub default_stream_sink_codec: CodecMode,
    pub default_rust_opaque_codec: RustOpaqueCodecMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct RustInputNamespacePack {
    rust_input_namespace_prefices: Vec<Namespace>,
}

impl RustInputNamespacePack {
    pub(crate) fn new(rust_input_namespace_prefices: Vec<Namespace>) -> Self {
        Self {
            rust_input_namespace_prefices,
        }
    }

    pub fn is_interest(&self, namespace: &Namespace) -> bool {
        (self.rust_input_namespace_prefices.iter()).any(|prefix| prefix.is_prefix_of(namespace))
    }
}
