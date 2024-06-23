use crate::if_then_some;
use itertools::Itertools;

pub(crate) fn parse_generics_info(generics: &syn::Generics) -> GenericsInfo {
    if generics.params.is_empty() {
        return GenericsInfo::Empty;
    }

    let lifetime_params = (generics.params.iter())
        .filter_map(
            |param| if_then_some!(let syn::GenericParam::Lifetime(inner) = param, inner.to_owned()),
        )
        .collect_vec();

    if lifetime_params.len() == generics.params.len() {
        GenericsInfo::LifetimeOnly
    } else {
        GenericsInfo::Unsupported
    }
}

pub(crate) enum GenericsInfo {
    Empty,
    LifetimeOnly,
    Unsupported,
}

pub(crate) fn should_ignore_because_generics(
    generics: &syn::Generics,
    enable_lifetime: bool,
) -> bool {
    match parse_generics_info(generics) {
        GenericsInfo::Empty => false,
        GenericsInfo::LifetimeOnly => !enable_lifetime,
        GenericsInfo::Unsupported => true,
    }
}
