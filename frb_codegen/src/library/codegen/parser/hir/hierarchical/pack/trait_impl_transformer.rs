use crate::codegen::ir::hir::hierarchical::function::{
    GeneralizedItemFn, HirFunction, HirFunctionOwner,
};
use crate::codegen::ir::hir::hierarchical::module::HirModule;
use crate::codegen::ir::hir::hierarchical::pack::HirPack;
use crate::codegen::ir::hir::hierarchical::traits::HirTrait;
use crate::codegen::parser::hir::hierarchical::function::parse_syn_item_impl;
use crate::if_then_some;
use crate::utils::namespace::Namespace;
use itertools::{concat, Itertools};
use std::collections::HashMap;
use syn::{ItemImpl, TraitItem};

pub(super) fn transform(mut pack: HirPack) -> anyhow::Result<HirPack> {
    let trait_map = collect_traits(&pack);
    pack.visit_mut(&mut |module| {
        (module.content.functions).extend(compute_methods(module, &trait_map));
    });
    Ok(pack)
}

fn collect_traits(pack: &HirPack) -> HashMap<String, HirTrait> {
    let mut traits = vec![];
    pack.visit(&mut |module| traits.extend(module.content.traits.clone()));
    traits
        .into_iter()
        .map(|t| (t.item_trait.ident.to_string(), t))
        .collect()
}

fn compute_methods(module: &HirModule, trait_map: &HashMap<String, HirTrait>) -> Vec<HirFunction> {
    (module.content.trait_impls.iter())
        .flat_map(|trait_impl| {
            let namespace = &trait_impl.namespace;

            let trait_name_raw = &trait_impl.item_impl.trait_.as_ref().unwrap().1;
            let trait_name = trait_name_raw.segments.last().unwrap().ident.to_string();
            // if BLACKLIST_TRAIT_NAMES.contains(&&trait_name[..]) {
            //     return vec![];
            // }

            let trait_def = trait_map.get(&trait_name);
            // Only parse impl of known traits by default, otherwise things like `lazy_staic!`
            // will introduce a ton of unwanted trait impls.
            if trait_def.is_none() {
                return vec![];
            }

            let impl_functions = parse_syn_item_impl(&trait_impl.item_impl, namespace, Some(TODO));
            let def_functions = trait_def
                .map(|t| parse_trait_def_functions(t, &trait_impl.item_impl, namespace))
                .unwrap_or_default();

            concat([impl_functions, def_functions])
                .into_iter()
                .unique_by(|f| f.owner_and_name())
                .collect_vec()
        })
        .collect_vec()
}

fn parse_trait_def_functions(
    trait_def: &HirTrait,
    item_impl: &ItemImpl,
    namespace: &Namespace,
) -> Vec<HirFunction> {
    (trait_def.item_trait.items.iter())
        .filter_map(
            |item| if_then_some!(let TraitItem::Fn(ref trait_item_fn) = item, trait_item_fn),
        )
        .map(|trait_item_fn| HirFunction {
            namespace: namespace.clone(),
            owner: HirFunctionOwner::Method {
                item_impl: item_impl.to_owned(),
                trait_def_name: Some(TODO),
            },
            item_fn: GeneralizedItemFn::TraitItemFn(trait_item_fn.to_owned()),
        })
        .collect_vec()
}

// const BLACKLIST_TRAIT_NAMES: [&str; 4] = ["Clone", "Default", "Debug", "Deref"];
