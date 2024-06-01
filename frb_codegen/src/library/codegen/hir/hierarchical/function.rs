use crate::codegen::ir::namespace::Namespace;
use proc_macro2::Span;
use std::path::PathBuf;
use syn::spanned::Spanned;
use syn::{Attribute, ImplItemFn, ItemFn, ItemImpl, Signature, Visibility};

pub(crate) struct HirFunction {
    pub(crate) namespace: Namespace,
    pub(crate) inner: HirFunctionInner,
}

#[derive(Debug, Clone)]
pub(crate) enum HirFunctionInner {
    Function {
        item_fn: ItemFn,
    },
    Method {
        item_impl: ItemImpl,
        impl_item_fn: ImplItemFn,
    },
}

impl HirFunctionInner {
    pub(crate) fn sig(&self) -> &Signature {
        match self {
            HirFunctionInner::Function { item_fn } => &item_fn.sig,
            HirFunctionInner::Method { impl_item_fn, .. } => &impl_item_fn.sig,
        }
    }

    pub(crate) fn attrs(&self) -> &Vec<Attribute> {
        match self {
            HirFunctionInner::Function { item_fn } => &item_fn.attrs,
            HirFunctionInner::Method { impl_item_fn, .. } => &impl_item_fn.attrs,
        }
    }

    pub(crate) fn span(&self) -> Span {
        match self {
            HirFunctionInner::Function { item_fn } => item_fn.span(),
            HirFunctionInner::Method { impl_item_fn, .. } => impl_item_fn.span(),
        }
    }

    pub(crate) fn vis(&self) -> &Visibility {
        match self {
            HirFunctionInner::Function { item_fn } => &item_fn.vis,
            HirFunctionInner::Method { impl_item_fn, .. } => &impl_item_fn.vis,
        }
    }
}
