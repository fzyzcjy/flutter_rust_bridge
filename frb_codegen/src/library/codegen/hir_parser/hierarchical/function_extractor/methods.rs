use crate::codegen::parser::function_extractor::structs::HirFunction;
use syn::{File, ImplItem, Item};

pub(super) fn extract_methods_from_syn_items(
    items: &[syn::Item],
) -> anyhow::Result<Vec<HirFunction>> {
    let mut src_fns = Vec::new();

    for item in items.iter() {
        if let Item::Impl(ref item_impl) = item {
            for item in &item_impl.items {
                if let ImplItem::Fn(impl_item_fn) = item {
                    src_fns.push(HirFunction::Method {
                        item_impl: item_impl.clone(),
                        impl_item_fn: impl_item_fn.clone(),
                    });
                }
            }
        }
    }

    Ok(src_fns)
}
