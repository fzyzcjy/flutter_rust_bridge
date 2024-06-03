use crate::codegen::ir::hir::hierarchical::function::{HirFunction, HirFunctionInner};
use crate::if_then_some;
use crate::utils::namespace::Namespace;
use itertools::Itertools;
use syn::{ImplItem, ImplItemFn, ItemFn, ItemImpl};

pub(crate) fn parse_syn_item_fn(item_fn: &ItemFn, namespace: &Namespace) -> HirFunction {
    HirFunction {
        namespace: namespace.clone(),
        inner: HirFunctionInner::Function {
            item_fn: item_fn.to_owned(),
        },
    }
}

pub(crate) fn parse_syn_item_impl(item_impl: &ItemImpl, namespace: &Namespace) -> Vec<HirFunction> {
    (item_impl.items.iter())
        .filter_map(|item| parse_syn_impl_item(impl_item))
        .map(|impl_item_fn| parse_syn_impl_item_fn(impl_item_fn, namespace))
        .collect_vec()
}

pub(crate) fn parse_syn_impl_item(
    impl_item: ImplItem,
    namespace: &Namespace,
) -> Option<HirFunction> {
    if let ImplItem::Fn(impl_item_fn) = impl_item {
        Some(HirFunction {
            namespace: namespace.clone(),
            inner: HirFunctionInner::Method {
                item_impl: item_impl.clone(),
                impl_item_fn: impl_item_fn.clone(),
            },
        })
    } else {
        None
    }
}
