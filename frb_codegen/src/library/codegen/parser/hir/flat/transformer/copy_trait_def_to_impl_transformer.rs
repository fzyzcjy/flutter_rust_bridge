use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::flat::trait_impl::HirFlatTraitImpl;

pub(crate) fn transform(mut pack: HirFlatPack) -> anyhow::Result<HirFlatPack> {
    for trait_impl in &pack.trait_impls {
        pack.functions.extend(compute_functions(trait_impl, &pack));
    }
    Ok(pack)
}

fn compute_functions(trait_impl: &HirFlatTraitImpl, pack: &HirFlatPack) -> Vec<HirFlatFunction> {
    TODO;
}
