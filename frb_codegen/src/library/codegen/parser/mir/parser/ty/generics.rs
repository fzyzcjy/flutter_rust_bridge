use crate::if_then_some;
use anyhow::{Context, Result};

pub(crate) fn parse_generic_lifetimes(generics: &syn::Generics) -> Result<Vec<syn::LifetimeParam>> {
    (generics.params.iter())
        .map(|param| {
            if_then_some!(let syn::GenericParam::Lifetime(inner) = param, inner.to_owned())
                .context("only support generic lifetime")
        })
        .collect()
}
