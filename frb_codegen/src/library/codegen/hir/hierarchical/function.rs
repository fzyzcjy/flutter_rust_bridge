use proc_macro2::Span;
use std::path::PathBuf;
use syn::spanned::Spanned;
use syn::{Attribute, ImplItemFn, ItemFn, ItemImpl, Signature, Visibility};

pub(crate) struct PathAndItemFn {
    pub(crate) path: PathBuf,
    pub(crate) generalized_item_fn: HirFunction,
}

#[derive(Debug, Clone)]
pub(crate) enum HirFunction {
    Function {
        item_fn: ItemFn,
    },
    Method {
        item_impl: ItemImpl,
        impl_item_fn: ImplItemFn,
    },
}

impl HirFunction {
    pub(crate) fn sig(&self) -> &Signature {
        match self {
            HirFunction::Function { item_fn } => &item_fn.sig,
            HirFunction::Method { impl_item_fn, .. } => &impl_item_fn.sig,
        }
    }

    pub(crate) fn attrs(&self) -> &Vec<Attribute> {
        match self {
            HirFunction::Function { item_fn } => &item_fn.attrs,
            HirFunction::Method { impl_item_fn, .. } => &impl_item_fn.attrs,
        }
    }

    pub(crate) fn span(&self) -> Span {
        match self {
            HirFunction::Function { item_fn } => item_fn.span(),
            HirFunction::Method { impl_item_fn, .. } => impl_item_fn.span(),
        }
    }

    pub(crate) fn vis(&self) -> &Visibility {
        match self {
            HirFunction::Function { item_fn } => &item_fn.vis,
            HirFunction::Method { impl_item_fn, .. } => &impl_item_fn.vis,
        }
    }
}
