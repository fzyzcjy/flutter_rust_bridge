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

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks names, attributes, signatures, visibility, and mutation access.
    #[test]
    fn generalized_item_fn_exposes_shared_accessors() {
        let mut functions = [
            GeneralizedItemFn::ItemFn(syn::parse_str("#[inline] pub fn item_fn() {}").unwrap()),
            GeneralizedItemFn::ImplItemFn(syn::parse_str("#[inline] pub fn impl_fn() {}").unwrap()),
            GeneralizedItemFn::TraitItemFn(syn::parse_str("#[inline] fn trait_fn();").unwrap()),
        ];

        assert_eq!(functions[0].name(), "item_fn");
        assert_eq!(functions[1].name(), "impl_fn");
        assert_eq!(functions[2].name(), "trait_fn");
        assert!(functions.iter().all(|function| function.attrs().len() == 1));
        assert!(functions.iter().all(|function| function.sig().ident != ""));
        assert!(functions[0].vis_raw().is_some());
        assert!(functions[1].vis_raw().is_some());
        assert!(functions[2].vis_raw().is_none());

        functions[0].attrs_mut().clear();
        assert!(functions[0].attrs().is_empty());
    }
}
