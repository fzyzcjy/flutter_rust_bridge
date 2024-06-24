use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::{Attribute, ImplItemFn, ItemFn, Signature, TraitItemFn, Visibility};

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone)]
pub(crate) enum GeneralizedItemFn {
    ItemFn(ItemFn),
    ImplItemFn(ImplItemFn),
    TraitItemFn(TraitItemFn),
}

impl GeneralizedItemFn {
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

    /// NOTE: When it is `ImplItemFn`, but it is in a `impl SomeTrait for SomeType`,
    /// it will always be `inherited` even if it should be public
    pub(crate) fn vis_raw(&self) -> Option<&Visibility> {
        match self {
            Self::ItemFn(inner) => Some(&inner.vis),
            Self::ImplItemFn(inner) => Some(&inner.vis),
            Self::TraitItemFn(_) => None,
        }
    }
}
