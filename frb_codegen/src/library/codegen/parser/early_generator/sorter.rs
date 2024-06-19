use crate::codegen::ir::early_generator::pack::IrEarlyGeneratorPack;
use crate::codegen::parser::hir::flat::transformer::sort_transformer::sort_hir_flat_pack;

pub(crate) fn generate(pack: &mut IrEarlyGeneratorPack) {
    sort_hir_flat_pack(&mut pack.hir_flat_pack);
}
