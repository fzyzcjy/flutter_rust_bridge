use crate::codegen::ir::hir::flat::function::{HirFlatFunction, HirFlatFunctionOwner};
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::misc::skip::{IrSkip, IrSkipReason, IrValueOrSkip};
use crate::utils::namespace::NamespacedName;
use itertools::Itertools;
use std::collections::HashSet;

pub(crate) fn transform(mut pack: HirFlatPack) -> anyhow::Result<HirFlatPack> {
    let good_trait_names: HashSet<String> =
        pack.traits.iter().map(|t| t.name.name.clone()).collect();

    let (funcs, skips) = IrValueOrSkip::split(
        (pack.functions.drain(..))
            .map(|f| {
                if should_retain(&f, &good_trait_names) {
                    IrValueOrSkip::Value(f)
                } else {
                    IrValueOrSkip::Skip(IrSkip {
                        name: NamespacedName::new(f.namespace.clone(), f.item_fn.name()),
                        reason: IrSkipReason::TODO,
                    })
                }
            })
            .collect_vec(),
    );
    pack.functions = funcs;
    pack.skips += skips;

    Ok(pack)
}

fn should_retain(f: &HirFlatFunction, good_trait_names: &HashSet<String>) -> bool {
    if let HirFlatFunctionOwner::StructOrEnum {
        trait_def_name: Some(trait_def_name),
        ..
    } = &f.owner
    {
        good_trait_names.contains(trait_def_name)
    } else {
        true
    }
}
