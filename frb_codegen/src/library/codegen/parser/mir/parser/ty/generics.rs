use crate::if_then_some;
use itertools::Itertools;

pub(crate) fn parse_generics_info(generics: &syn::Generics) -> GenericsInfo {
    if generics.params.len() == 0 {
        return GenericsInfo::Empty;
    }

    let lifetime_params = (generics.params.iter())
        .filter_map(
            |param| if_then_some!(let syn::GenericParam::Lifetime(inner) = param, inner.to_owned()),
        )
        .collect_vec();

    if lifetime_params.len() == generics.params.len() {
        GenericsInfo::LifetimeOnly(lifetime_params)
    } else {
        GenericsInfo::Unsupported
    }
}

pub(crate) enum GenericsInfo {
    Empty,
    LifetimeOnly(Vec<syn::LifetimeParam>),
    Unsupported,
}
