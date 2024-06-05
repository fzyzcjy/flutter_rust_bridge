use crate::codegen::ir::hir::flat::function::{HirFlatFunction, HirFlatFunctionOwner};
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::flat::trait_impl::HirFlatTraitImpl;
use crate::codegen::ir::hir::misc::item_fn::GeneralizedItemFn;
use crate::codegen::ir::hir::tree::module::HirTreeModuleMeta;
use crate::if_then_some;
use crate::utils::namespace::{Namespace, NamespacedName};
use itertools::Itertools;
use syn::{ImplItem, ItemImpl, ItemStruct};

pub(crate) fn parse_syn_item_impl(
    target: &mut HirFlatPack,
    item_impl: ItemImpl,
    meta: &HirTreeModuleMeta,
) {
    let trait_name = parse_trait_name(&item_impl);

    if let Some(trait_name) = &trait_name {
        (target.trait_impls).push(parse_trait_impl(&item_impl, trait_name));
    }
    (target.functions).extend(parse_functions(item_impl, meta, &trait_name));
}

fn parse_trait_name(item_impl: &ItemImpl) -> Option<String> {
    (item_impl.trait_.as_ref())
        .map(|t| t.1)
        .map(|t| t.segments.last().unwrap().ident.to_string())
}

fn parse_functions(
    item_impl: ItemImpl,
    meta: &HirTreeModuleMeta,
    trait_def_name: &Option<String>,
) -> Vec<HirFlatFunction> {
    (item_impl.items.into_iter())
        .filter_map(|item| if_then_some!(let ImplItem::Fn(impl_item_fn) = item, impl_item_fn))
        .map(|impl_item_fn| HirFlatFunction {
            namespace: meta.namespace.clone(),
            owner: HirFlatFunctionOwner::StructOrEnum {
                impl_ty: *item_impl.self_ty.clone(),
                trait_def_name: trait_def_name.clone(),
            },
            item_fn: GeneralizedItemFn::ImplItemFn(impl_item_fn),
        })
        .collect_vec()
}

fn parse_trait_impl(item_impl: &ItemImpl, trait_name: &str) -> HirFlatTraitImpl {
    HirFlatTraitImpl {
        trait_name: trait_name.to_owned(),
        impl_ty: *item_impl.self_ty.clone(),
    }
}
