use crate::codegen::ir::hir::flat::function::{HirFlatFunction, HirFlatFunctionOwner};
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::flat::trait_impl::HirFlatTraitImpl;
use itertools::Itertools;
use std::collections::HashSet;

pub(crate) fn transform(mut pack: HirFlatPack) -> anyhow::Result<HirFlatPack> {
    let good_trait_names: HashSet<String> =
        pack.traits.iter().map(|t| t.name.name.clone()).collect();

    pack.functions = (pack.functions.drain(..))
        .filter(|f| {
            if let HirFlatFunctionOwner::StructOrEnum {
                trait_def_name: Some(trait_def_name),
                ..
            } = &f.owner
            {
                good_trait_names.contains(trait_def_name)
            } else {
                true
            }
        })
        .collect_vec();
    Ok(pack)
}
