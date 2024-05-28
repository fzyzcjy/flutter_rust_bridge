use crate::codegen::parser::function_extractor::structs::GeneralizedItemFn;
use syn::{File, ImplItem, Item};

pub(super) fn extract_methods_from_file(file: &File) -> anyhow::Result<Vec<GeneralizedItemFn>> {
    let mut src_fns = Vec::new();

    for item in file.items.iter() {
        if let Item::Impl(ref item_impl) = item {
            for item in &item_impl.items {
                if let ImplItem::Fn(impl_item_fn) = item {
                    src_fns.push(GeneralizedItemFn::Method {
                        item_impl: item_impl.clone(),
                        impl_item_fn: impl_item_fn.clone(),
                    });
                }
            }
        }
    }

    Ok(src_fns)
}
