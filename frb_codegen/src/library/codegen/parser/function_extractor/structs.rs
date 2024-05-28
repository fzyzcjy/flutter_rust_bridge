use proc_macro2::Span;
use std::path::PathBuf;
use syn::spanned::Spanned;
use syn::{Attribute, ImplItemFn, ItemFn, ItemImpl, Signature, Visibility};

pub(crate) struct PathAndItemFn {
    pub(crate) path: PathBuf,
    pub(crate) generalized_item_fn: GeneralizedItemFn,
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

    pub(crate) fn vis(&self) -> Visibility {
        match self {
            GeneralizedItemFn::Function { item_fn } => item_fn.vis,
            GeneralizedItemFn::Method { impl_item_fn, .. } => impl_item_fn.vis,
        }
    }
}
