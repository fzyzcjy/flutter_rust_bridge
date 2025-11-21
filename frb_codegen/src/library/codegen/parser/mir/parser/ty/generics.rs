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
        // Extract type parameter names (e.g., ["T"] for <T>)
        let type_params = generics
            .params
            .iter()
            .filter_map(|param| {
                if let syn::GenericParam::Type(ty) = param {
                    Some(ty.ident.to_string())
                } else {
                    None
                }
            })
            .collect_vec();
        
        if !type_params.is_empty() {
            GenericsInfo::Generic { params: type_params }
        } else {
            GenericsInfo::Unsupported
        }
    }
}

pub(crate) enum GenericsInfo {
    Empty,
    LifetimeOnly,
    Generic {
        params: Vec<String>,
    },
    Unsupported,
}

pub(crate) fn should_ignore_because_generics(
    generics: &syn::Generics,
    enable_lifetime: bool,
) -> bool {
    match parse_generics_info(generics) {
        GenericsInfo::Empty => false,
        GenericsInfo::LifetimeOnly => !enable_lifetime,
        GenericsInfo::Generic { .. } => false, // Allow generic enums/structs (they'll be stored as templates)
        GenericsInfo::Unsupported => true,
    }
}

pub(crate) fn is_generic_template(generics: &syn::Generics) -> bool {
    matches!(parse_generics_info(generics), GenericsInfo::Generic { .. })
}
