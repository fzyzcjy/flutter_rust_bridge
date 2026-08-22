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

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> syn::Generics {
        syn::parse_str::<syn::ItemStruct>(input).unwrap().generics
    }

    /// Classifies declarations without generic parameters as empty.
    #[test]
    fn parse_generics_info_classifies_empty_generics() {
        assert!(matches!(
            parse_generics_info(&parse("struct Value;")),
            GenericsInfo::Empty
        ));
    }

    /// Allows lifetime-only generics only when lifetime support is enabled.
    #[test]
    fn should_ignore_lifetime_only_generics_depends_on_feature() {
        let generics = parse("struct Borrowed<'a>(&'a str);");

        assert!(matches!(
            parse_generics_info(&generics),
            GenericsInfo::LifetimeOnly
        ));
        assert!(should_ignore_because_generics(&generics, false));
        assert!(!should_ignore_because_generics(&generics, true));
    }

    /// Rejects declarations with type or const generic parameters.
    #[test]
    fn should_ignore_unsupported_generic_parameters() {
        let type_generics = parse("struct Container<T>(T);");
        let const_generics = parse("struct Buffer<const N: usize>([u8; N]);");

        assert!(matches!(
            parse_generics_info(&type_generics),
            GenericsInfo::Unsupported
        ));
        assert!(matches!(
            parse_generics_info(&const_generics),
            GenericsInfo::Unsupported
        ));
        assert!(should_ignore_because_generics(&type_generics, true));
        assert!(should_ignore_because_generics(&const_generics, true));
    }
}
