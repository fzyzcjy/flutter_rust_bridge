use crate::utils::namespace::Namespace;
use proc_macro2::Span;
use serde::Serialize;
use syn::spanned::Spanned;
use syn::{Attribute, ImplItemFn, ItemFn, ItemImpl, ItemTrait, Signature, TraitItemFn, Visibility};

#[derive(Debug, Clone, Serialize)]
pub(crate) struct HirFunction {
    pub(crate) namespace: Namespace,
    #[serde(skip_serializing)]
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
    TraitMethod {
        item_trait: ItemTrait,
        trait_item_fn: TraitItemFn,
    },
}

impl HirFunction {
    pub(crate) fn with_namespace(&self, namespace: Namespace) -> Self {
        Self {
            namespace,
            ..self.clone()
        }
    }
}

impl HirFunctionInner {
    pub(crate) fn sig(&self) -> &Signature {
        match self {
            HirFunctionInner::Function { item_fn } => &item_fn.sig,
            HirFunctionInner::Method { impl_item_fn, .. } => &impl_item_fn.sig,
        }
    }

    pub(crate) fn simple_owner(&self) -> Option<String> {
        match &self {
            HirFunctionInner::Method { item_impl, .. } => ty_to_string(&item_impl.self_ty),
            HirFunctionInner::TraitMethod { .. } => TODO,
            _ => return None,
        }
    }

    pub(crate) fn name(&self) -> String {
        self.sig().ident.to_string()
    }

    pub(crate) fn owner_and_name(&self) -> SimpleOwnerAndName {
        (self.simple_owner(), self.name())
    }

    pub(crate) fn attrs(&self) -> &Vec<Attribute> {
        match self {
            HirFunctionInner::Function { item_fn } => &item_fn.attrs,
            HirFunctionInner::Method { impl_item_fn, .. } => &impl_item_fn.attrs,
        }
    }

    pub(crate) fn attrs_mut(&mut self) -> &mut Vec<Attribute> {
        match self {
            HirFunctionInner::Function { item_fn } => &mut item_fn.attrs,
            HirFunctionInner::Method { impl_item_fn, .. } => &mut impl_item_fn.attrs,
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

pub(crate) type SimpleOwnerAndName = (Option<String>, String);

fn ty_to_string(ty: &syn::Type) -> String {
    quote::quote!(#ty).to_string()
}
