use crate::codegen::ir::hir::hierarchical::function::{HirFunction, HirFunctionInner};
use crate::if_then_some;
use crate::utils::namespace::Namespace;
use itertools::Itertools;
use syn::{ImplItem, Item};

pub(super) fn parse_generalized_functions(
    items: &[syn::Item],
    namespace: &Namespace,
) -> anyhow::Result<Vec<HirFunction>> {
    let item_fns = [parse_functions(items), parse_methods(items)?].concat();
    let ans = (item_fns.into_iter())
        .map(|inner| HirFunction {
            namespace: namespace.clone(),
            inner,
        })
        .collect_vec();
    Ok(ans)
}

fn parse_functions(items: &[syn::Item]) -> Vec<HirFunctionInner> {
    (items.iter())
        .filter_map(|item| if_then_some!(let Item::Fn(ref item_fn) = item, item_fn))
        .cloned()
        .map(|item_fn| HirFunctionInner::Function { item_fn })
        .collect_vec()
}

fn parse_methods(items: &[syn::Item]) -> anyhow::Result<Vec<HirFunctionInner>> {
    let mut src_fns = Vec::new();
    for item in items.iter() {
        if let Item::Impl(ref item_impl) = item {
            for item in &item_impl.items {
                if let ImplItem::Fn(impl_item_fn) = item {
                    src_fns.push(HirFunctionInner::Method {
                        item_impl: item_impl.clone(),
                        impl_item_fn: impl_item_fn.clone(),
                    });
                }
            }
        }
    }
    Ok(src_fns)
}
