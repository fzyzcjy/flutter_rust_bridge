use crate::codegen::ir::hir::flat::function::{HirFlatFunction, HirFlatFunctionOwner};
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::flat::traits::HirFlatTrait;
use crate::codegen::ir::hir::misc::item_fn::GeneralizedItemFn;
use crate::codegen::ir::hir::naive_flat::item::HirNaiveFlatItemMeta;
use crate::if_then_some;
use crate::utils::namespace::NamespacedName;
use itertools::Itertools;
use syn::ItemTrait;
use syn::TraitItem;

pub(crate) fn parse_syn_item_trait(
    target: &mut HirFlatPack,
    item_trait: ItemTrait,
    meta: &HirNaiveFlatItemMeta,
) {
    let trait_name = NamespacedName::new(meta.namespace.clone(), item_trait.ident.to_string());
    target.traits.push(HirFlatTrait {
        name: trait_name.clone(),
        attrs: item_trait.attrs.clone(),
        sources: meta.sources.clone(),
    });
    (target.functions).extend(parse_functions(item_trait, meta, &trait_name));
}

fn parse_functions(
    item_trait: ItemTrait,
    meta: &HirNaiveFlatItemMeta,
    trait_def_name: &NamespacedName,
) -> Vec<HirFlatFunction> {
    (item_trait.items.into_iter())
        .filter_map(|item| if_then_some!(let TraitItem::Fn(trait_item_fn) = item, trait_item_fn))
        .map(|trait_item_fn| HirFlatFunction {
            namespace: meta.namespace.clone(),
            owner: HirFlatFunctionOwner::TraitDef {
                trait_def_name: trait_def_name.to_owned(),
            },
            item_fn: GeneralizedItemFn::TraitItemFn(trait_item_fn),
            sources: meta.sources.clone(),
        })
        .collect_vec()
}
