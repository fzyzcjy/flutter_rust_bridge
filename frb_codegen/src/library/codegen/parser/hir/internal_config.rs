use crate::codegen::generator::codec::structs::{CodecMode, CodecModePack};
use crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode;
use crate::codegen::parser::mir::internal_config::RustInputNamespacePack;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ParserHirInternalConfig {
    pub rust_input_namespace_pack: RustInputNamespacePack,
}
