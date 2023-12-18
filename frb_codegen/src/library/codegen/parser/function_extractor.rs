use crate::if_then_some;
use itertools::Itertools;
use quote::__private::Span;
use std::path::PathBuf;
use syn::spanned::Spanned;
use syn::File;
use syn::*;

pub(super) struct PathAndItemFn {
    pub(super) path: PathBuf,
    pub(super) generalized_item_fn: GeneralizedItemFn,
}

#[derive(Debug, Clone)]
pub(crate) enum GeneralizedItemFn {
    Function {
        item_fn: ItemFn,
    },
    Method {
        item_impl: ItemImpl,
        impl_item_fn: ImplItemFn,
    },
}

impl GeneralizedItemFn {
    pub(crate) fn sig(&self) -> &Signature {
        match self {
            GeneralizedItemFn::Function { item_fn } => &item_fn.sig,
            GeneralizedItemFn::Method { impl_item_fn, .. } => &impl_item_fn.sig,
        }
    }

    pub(crate) fn attrs(&self) -> &Vec<Attribute> {
        match self {
            GeneralizedItemFn::Function { item_fn } => &item_fn.attrs,
            GeneralizedItemFn::Method { impl_item_fn, .. } => &impl_item_fn.attrs,
        }
    }

    pub(crate) fn span(&self) -> Span {
        match self {
            GeneralizedItemFn::Function { item_fn } => item_fn.span(),
            GeneralizedItemFn::Method { impl_item_fn, .. } => impl_item_fn.span(),
        }
    }
}

pub(super) fn extract_generalized_functions_from_file(
    file: &File,
    path: &std::path::Path,
) -> anyhow::Result<Vec<PathAndItemFn>> {
    let item_fns = [
        extract_fns_from_file(file),
        extract_methods_from_file(file)?,
    ]
    .concat();
    let ans = item_fns
        .into_iter()
        .map(|generalized_item_fn| PathAndItemFn {
            path: path.to_owned(),
            generalized_item_fn,
        })
        .collect_vec();
    Ok(ans)
}

fn extract_fns_from_file(file: &File) -> Vec<GeneralizedItemFn> {
    file.items
        .iter()
        .filter_map(|item| if_then_some!(let Item::Fn(ref item_fn) = item, item_fn))
        .filter(|item_fn| matches!(item_fn.vis, Visibility::Public(_)))
        .cloned()
        .map(|item_fn| GeneralizedItemFn::Function { item_fn })
        .collect_vec()
}

fn extract_methods_from_file(file: &File) -> anyhow::Result<Vec<GeneralizedItemFn>> {
    let mut src_fns = Vec::new();

    for item in file.items.iter() {
        if let Item::Impl(ref item_impl) = item {
            for item in &item_impl.items {
                if let ImplItem::Fn(impl_item_fn) = item {
                    if let Visibility::Public(_) = &impl_item_fn.vis {
                        src_fns.push(GeneralizedItemFn::Method {
                            item_impl: item_impl.clone(),
                            impl_item_fn: impl_item_fn.clone(),
                        });
                    }
                }
            }
        }
    }

    Ok(src_fns)
}
