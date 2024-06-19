use crate::codegen::ir::early_generator::pack::IrEarlyGeneratorPack;
use crate::codegen::parser::hir::flat::transformer::sort_transformer::sort_hir_flat_pack;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;

pub(crate) fn generate(pack: &mut IrEarlyGeneratorPack) {
    sort_hir_flat_pack(&mut pack.hir_flat_pack);

    (pack.proxied_types).sort_by_cached_key(|x| x.original_ty.safe_ident());
    (pack.trait_def_infos).sort_by_cached_key(|x| x.trait_def_name.clone());
}
