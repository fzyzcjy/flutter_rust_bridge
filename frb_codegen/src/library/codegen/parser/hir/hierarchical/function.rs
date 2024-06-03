use crate::codegen::ir::hir::hierarchical::function::{GeneralItemFn, HirFunction};
use crate::if_then_some;
use crate::utils::namespace::Namespace;
use itertools::Itertools;
use syn::{ImplItem, ItemFn, ItemImpl};

pub(crate) fn parse_syn_item_fn(item_fn: &ItemFn, namespace: &Namespace) -> HirFunction {
    HirFunction {
        namespace: namespace.clone(),
        item_impl: None,
        item_fn: GeneralItemFn::ItemFn(item_fn.to_owned()),
    }
}

pub(crate) fn parse_syn_item_impl(
    item_impl: &ItemImpl,
    namespace: &Namespace,
    owned_by_trait: bool,
) -> Vec<HirFunction> {
    (item_impl.items.iter())
        .filter_map(|item| if_then_some!(let ImplItem::Fn(ref impl_item_fn) = item, impl_item_fn))
        .map(|impl_item_fn| HirFunction {
            namespace: namespace.clone(),
            item_impl: Some(item_impl.clone()),
            item_fn: GeneralItemFn::ImplItemFn {
                inner: impl_item_fn.clone(),
                owned_by_trait,
            },
        })
        .collect_vec()
}
