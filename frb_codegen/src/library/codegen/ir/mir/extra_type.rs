use crate::codegen::generator::codec::structs::CodecModePack;
use crate::codegen::ir::mir::ty::MirType;

crate::mir! {
pub struct MirExtraType {
    pub ty: MirType,
    pub codec_mode_pack: CodecModePack,
}
}
