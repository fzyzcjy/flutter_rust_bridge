use crate::codegen::ir::hir::hierarchical::function::{
    GeneralizedItemFn, HirFunction, HirFunctionOwner,
};
use crate::if_then_some;
use crate::utils::namespace::{Namespace, NamespacedName};
use itertools::Itertools;
use syn::{ImplItem, ItemFn, ItemImpl};

pub(crate) fn parse_syn_item_fn(item_fn: &ItemFn, namespace: &Namespace) -> HirFunction {
    HirFunction {
        namespace: namespace.clone(),
        owner: HirFunctionOwner::Function,
        item_fn: GeneralizedItemFn::ItemFn(item_fn.to_owned()),
    }
}

pub(crate) fn parse_syn_item_impl(
    item_impl: &ItemImpl,
    namespace: &Namespace,
    trait_def_name: Option<NamespacedName>,
) -> Vec<HirFunction> {
    (item_impl.items.iter())
        .filter_map(|item| if_then_some!(let ImplItem::Fn(ref impl_item_fn) = item, impl_item_fn))
        .map(|impl_item_fn| HirFunction {
            namespace: namespace.clone(),
            owner: HirFunctionOwner::Method {
                item_impl: item_impl.to_owned(),
                trait_def_name,
            },
            item_fn: GeneralizedItemFn::ImplItemFn(impl_item_fn.clone()),
        })
        .collect_vec()
}
