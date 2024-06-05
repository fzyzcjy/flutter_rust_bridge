use crate::codegen::ir::hir::flat::function::{HirFlatFunction, HirFlatFunctionOwner};
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::flat::trait_impl::HirFlatTraitImpl;
use crate::codegen::ir::hir::flat::traits::HirFlatTrait;
use crate::if_then_some;
use itertools::Itertools;
use std::collections::HashMap;

pub(crate) fn transform(mut pack: HirFlatPack) -> anyhow::Result<HirFlatPack> {
    for trait_impl in &pack.trait_impls {
        pack.functions.extend(compute_functions(trait_impl, &pack));
    }
    Ok(pack)
}

fn compute_functions(trait_impl: &HirFlatTraitImpl, pack: &HirFlatPack) -> Vec<HirFlatFunction> {
    (pack.functions.iter())
        .filter(|f| {
            TODO;
            TODO;
        })
        .map(|f| {
            TODO;
            TODO;
        })
        .collect_vec()
}
