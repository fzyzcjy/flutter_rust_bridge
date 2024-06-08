use crate::codegen::ir::hir::flat::pack::HirFlatPack;

#[derive(Debug, Clone, Default, serde::Serialize)]
pub(crate) struct IrEarlyGeneratorPack {
    pub hir_flat_pack: HirFlatPack,
    pub proxied_types: Vec<String>,
}
