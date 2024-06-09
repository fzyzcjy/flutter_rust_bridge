use crate::codegen::ir::early_generator::proxied_type::IrEarlyGeneratorProxiedType;
use crate::codegen::ir::early_generator::trait_def_info::IrEarlyGeneratorTraitDefInfo;
use crate::codegen::ir::hir::flat::pack::HirFlatPack;

#[derive(Debug, Clone, Default, serde::Serialize)]
pub(crate) struct IrEarlyGeneratorPack {
    pub hir_flat_pack: HirFlatPack,
    pub proxied_types: Vec<IrEarlyGeneratorProxiedType>,
    pub trait_def_infos: Vec<IrEarlyGeneratorTraitDefInfo>,
}
