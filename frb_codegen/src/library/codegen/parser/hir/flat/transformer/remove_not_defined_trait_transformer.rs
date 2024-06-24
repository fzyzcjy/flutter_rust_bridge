use crate::codegen::ir::hir::flat::function::{HirFlatFunction, HirFlatFunctionOwner};
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::misc::skip::{IrSkip, IrSkipReason, IrValueOrSkip};
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
use crate::utils::namespace::NamespacedName;
use itertools::Itertools;
use std::collections::HashSet;

pub(crate) fn transform(mut pack: HirFlatPack) -> anyhow::Result<HirFlatPack> {
    let good_trait_names: HashSet<String> =
        (pack.traits.iter().map(|t| t.name.name.clone())).collect();

    let (funcs, skips) = IrValueOrSkip::split(
        (pack.functions.drain(..))
            .map(|f| {
                if should_retain(&f, &good_trait_names) {
                    IrValueOrSkip::Value(f)
                } else {
                    IrValueOrSkip::Skip(IrSkip {
                        name: NamespacedName::new(f.namespace.clone(), f.item_fn.name()),
                        reason: IrSkipReason::IgnoreBecauseNotDefinedTrait,
                    })
                }
            })
            .collect_vec(),
    );
    pack.functions = funcs;
    pack.skips.extend(skips);

    Ok(pack)
}

fn should_retain(f: &HirFlatFunction, good_trait_names: &HashSet<String>) -> bool {
    if let HirFlatFunctionOwner::StructOrEnum {
        trait_def_name: Some(trait_def_name),
        ..
    } = &f.owner
    {
        good_trait_names.contains(trait_def_name)
            || WHITELIST_TRAIT_NAMES.contains(&&**trait_def_name)
            || has_frb_attributes(f)
    } else {
        true
    }
}

// https://github.com/fzyzcjy/flutter_rust_bridge/issues/2103#issuecomment-2178061684
fn has_frb_attributes(f: &HirFlatFunction) -> bool {
    let attrs = FrbAttributes::parse(f.item_fn.attrs()).unwrap();
    !attrs.is_empty()
}

pub(crate) const WHITELIST_TRAIT_NAMES: [&str; 1] = ["Default"];
