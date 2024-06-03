use crate::utils::namespace::Namespace;
use proc_macro2::Span;
use serde::Serialize;
use syn::spanned::Spanned;
use syn::{Attribute, ImplItemFn, ItemFn, ItemImpl, Signature, TraitItemFn, Visibility};

#[derive(Debug, Clone, Serialize)]
pub(crate) struct HirFunction {
    pub(crate) namespace: Namespace,
    /// Only exist for methods (and not exist for functions)
    #[serde(skip_serializing)]
    pub(crate) item_impl: Option<ItemImpl>,
    #[serde(skip_serializing)]
    pub(crate) item_fn: GeneralItemFn,
}

impl HirFunction {
    pub(crate) fn with_namespace(&self, namespace: Namespace) -> Self {
        Self {
            namespace,
            ..self.clone()
        }
    }

    pub(crate) fn simple_owner(&self) -> Option<String> {
        self.item_impl
            .map(|item_impl| ty_to_string(&item_impl.self_ty))
    }

    pub(crate) fn owner_and_name(&self) -> SimpleOwnerAndName {
        (self.simple_owner(), self.item_fn.name())
    }
}

#[derive(Debug, Clone)]
pub(crate) enum GeneralItemFn {
    ItemFn(ItemFn),
    ImplItemFn(ImplItemFn),
    TraitItemFn(TraitItemFn),
}

impl GeneralItemFn {
    pub(crate) fn sig(&self) -> &Signature {
        match self {
            Self::ItemFn(inner) => &inner.sig,
            Self::ImplItemFn(inner) => &inner.sig,
            Self::TraitItemFn(inner) => &inner.sig,
        }
    }

    pub(crate) fn name(&self) -> String {
        self.sig().ident.to_string()
    }

    pub(crate) fn attrs(&self) -> &Vec<Attribute> {
        match self {
            Self::ItemFn(inner) => &inner.attrs,
            Self::ImplItemFn(inner) => &inner.attrs,
            Self::TraitItemFn(inner) => &inner.attrs,
        }
    }

    pub(crate) fn attrs_mut(&mut self) -> &mut Vec<Attribute> {
        match self {
            Self::ItemFn(inner) => &mut inner.attrs,
            Self::ImplItemFn(inner) => &mut inner.attrs,
            Self::TraitItemFn(inner) => &mut inner.attrs,
        }
    }

    pub(crate) fn span(&self) -> Span {
        match self {
            Self::ItemFn(inner) => inner.span(),
            Self::ImplItemFn(inner) => inner.span(),
            Self::TraitItemFn(inner) => inner.span(),
        }
    }

    pub(crate) fn vis(&self) -> &Visibility {
        match self {
            Self::ItemFn(inner) => &inner.vis,
            Self::ImplItemFn(inner) => &inner.vis,
            Self::TraitItemFn(inner) => &inner.vis,
        }
    }
}

pub(crate) type SimpleOwnerAndName = (Option<String>, String);

fn ty_to_string(ty: &syn::Type) -> String {
    quote::quote!(#ty).to_string()
}
