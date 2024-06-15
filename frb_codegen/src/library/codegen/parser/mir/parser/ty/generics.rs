use crate::if_then_some;

pub(crate) fn parse_generic_lifetimes(generics: &syn::Generics) -> Option<Vec<syn::LifetimeParam>> {
    (generics.params.iter())
        .map(
            |param| if_then_some!(let syn::GenericParam::Lifetime(inner) = param, inner.to_owned()),
        )
        .collect()
}
