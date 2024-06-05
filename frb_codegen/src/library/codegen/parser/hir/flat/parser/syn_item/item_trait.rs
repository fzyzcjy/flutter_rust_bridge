use crate::codegen::ir::hir::misc::item_fn::GeneralizedItemFn;
use crate::codegen::ir::hir::flat::function::{HirFlatFunction, HirFlatFunctionOwner};
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::flat::traits::HirFlatTrait;
use crate::codegen::ir::hir::tree::module::HirTreeModuleMeta;
use crate::if_then_some;
use crate::utils::namespace::{Namespace, NamespacedName};
use syn::ItemTrait;
use syn::{ItemImpl, TraitItem};

pub(crate) fn parse_syn_item_trait(
    target: &mut HirFlatPack,
    item_trait: ItemTrait,
    meta: &HirTreeModuleMeta,
) {
    let trait_name = NamespacedName::new(namespace.clone(), item_trait.ident.to_string());
    target.traits.push(HirFlatTrait { name: trait_name });
    (target.functions).extend(parse_functions(item_trait, meta, &trait_name));
}

fn parse_functions(
    item_trait: ItemTrait,
    meta: &HirTreeModuleMeta,
    trait_def_name: &NamespacedName,
) -> Vec<HirFlatFunction> {
    (item_trait.items.into_iter())
        .filter_map(|item| if_then_some!(let TraitItem::Fn(trait_item_fn) = item, trait_item_fn))
        .map(|trait_item_fn| HirFlatFunction {
            namespace: meta.namespace.clone(),
            owner: HirFlatFunctionOwner::TraitDef {
                trait_def_name: Some(trait_def_name.to_owned()),
            },
            item_fn: GeneralizedItemFn::TraitItemFn(trait_item_fn),
        })
        .collect_vec()
}
