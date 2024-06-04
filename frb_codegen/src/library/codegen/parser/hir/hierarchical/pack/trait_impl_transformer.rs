use crate::codegen::ir::hir::hierarchical::function::{
    GeneralizedItemFn, HirFunction, HirFunctionOwner,
};
use crate::codegen::ir::hir::hierarchical::module::HirModule;
use crate::codegen::ir::hir::hierarchical::pack::HirPack;
use crate::codegen::ir::hir::hierarchical::traits::HirTrait;
use crate::codegen::parser::hir::flat::collect_traits;
use crate::codegen::parser::hir::hierarchical::function::parse_syn_item_impl;
use crate::if_then_some;
use crate::utils::namespace::{Namespace, NamespacedName};
use itertools::{concat, Itertools};
use std::collections::HashMap;
use syn::{ItemImpl, TraitItem};

pub(super) fn transform(mut pack: HirPack) -> anyhow::Result<HirPack> {
    let trait_map = (collect_traits(&pack).into_iter())
        .map(|(k, v)| (k, v.to_owned()))
        .collect::<HashMap<_, _>>();

    pack.visit_mut(&mut |module| {
        (module.content.functions).extend(compute_methods(module, &trait_map));
    });

    Ok(pack)
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

            // Only parse impl of known traits by default, otherwise things like `lazy_staic!`
            // will introduce a ton of unwanted trait impls.
            if let Some(trait_def) = trait_map.get(&trait_name) {
                let trait_def_name =
                    NamespacedName::new(trait_def.namespace.clone(), trait_name.clone());

                let impl_functions = parse_syn_item_impl(
                    &trait_impl.item_impl,
                    namespace,
                    Some(trait_def_name.clone()),
                );
                let def_functions = parse_trait_def_functions(
                    trait_def,
                    &trait_impl.item_impl,
                    namespace,
                    &trait_def_name,
                );

                concat([impl_functions, def_functions])
                    .into_iter()
                    .unique_by(|f| f.owner_and_name())
                    .collect_vec()
            } else {
                vec![]
            }
        })
        .collect_vec()
}

fn parse_trait_def_functions(
    trait_def: &HirTrait,
    item_impl: &ItemImpl,
    namespace: &Namespace,
    trait_def_name: &NamespacedName,
) -> Vec<HirFunction> {
    (trait_def.item_trait.items.iter())
        .filter_map(
            |item| if_then_some!(let TraitItem::Fn(ref trait_item_fn) = item, trait_item_fn),
        )
        .map(|trait_item_fn| HirFunction {
            namespace: namespace.clone(),
            owner: HirFunctionOwner::Method {
                item_impl: item_impl.to_owned(),
                trait_def_name: Some(trait_def_name.to_owned()),
            },
            item_fn: GeneralizedItemFn::TraitItemFn(trait_item_fn.to_owned()),
        })
        .collect_vec()
}

// const BLACKLIST_TRAIT_NAMES: [&str; 4] = ["Clone", "Default", "Debug", "Deref"];
