use crate::codegen::ir::hir::flat::function::{HirFlatFunction, HirFlatFunctionOwner};
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::flat::trait_impl::HirFlatTraitImpl;
use itertools::Itertools;

pub(crate) fn transform(mut pack: HirFlatPack) -> anyhow::Result<HirFlatPack> {
    pack.functions = (pack.functions.drain(..))
        .filter(|f| {
            if let HirFlatFunctionOwner::StructOrEnum {
                trait_def_name: Some(trait_def_name),
                ..
            } = &f.owner
            {
                TODO
            } else {
                true
            }
        })
        .collect_vec();
    Ok(pack)
}
