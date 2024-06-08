use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::mir::ty::MirType;

#[derive(Debug, Clone, Default, serde::Serialize)]
pub(crate) struct IrEarlyGeneratorPack {
    pub hir_flat_pack: HirFlatPack,
    pub proxied_types: Vec<MirType>,
}
