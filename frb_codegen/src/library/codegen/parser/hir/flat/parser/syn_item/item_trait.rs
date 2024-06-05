use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::flat::traits::HirFlatTrait;
use crate::codegen::ir::hir::hierarchical::traits::HirTrait;
use crate::codegen::ir::hir::tree::module::HirTreeModuleMeta;
use crate::if_then_some;
use crate::utils::namespace::{Namespace, NamespacedName};
use syn::ItemImpl;
use syn::ItemTrait;

pub(crate) fn parse_syn_item_trait(
    target: &mut HirFlatPack,
    item_trait: ItemTrait,
    meta: &HirTreeModuleMeta,
) -> HirTrait {
    target.traits.push(parse_trait(&item_trait, meta));
    target.functions.extend(TODO);
}

fn parse_trait(item_trait: &ItemTrait, meta: &HirTreeModuleMeta) -> HirFlatTrait {
    HirFlatTrait {
        name: NamespacedName::new(namespace.clone(), item_trait.ident.to_string()),
    }
}

fn parse_trait_def_functions(
    trait_def: &HirTrait,
    item_impl: &ItemImpl,
    namespace: &Namespace,
    trait_def_name: &NamespacedName,
) -> Vec<HirFlatFunction> {
    (trait_def.item_trait.items.iter())
        .filter_map(
            |item| if_then_some!(let TraitItem::Fn(ref trait_item_fn) = item, trait_item_fn),
        )
        .map(|trait_item_fn| HirFlatFunction {
            namespace: namespace.clone(),
            owner: HirFlatFunctionOwner::Method {
                item_impl: item_impl.to_owned(),
                trait_def_name: Some(trait_def_name.to_owned()),
            },
            item_fn: GeneralizedItemFn::TraitItemFn(trait_item_fn.to_owned()),
        })
        .collect_vec()
}
