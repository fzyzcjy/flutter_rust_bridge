use crate::if_then_some;
use itertools::Itertools;
use syn::{GenericArgument, Type};

pub(crate) struct LifetimeExtractor;

impl LifetimeExtractor {
    pub(crate) fn extract_skipping_static(ty: &Type) -> Vec<String> {
        (Self::extract(ty).into_iter())
            .filter(|lifetime| lifetime != LIFETIME_STATIC)
            .collect_vec()
    }

    fn extract(ty: &Type) -> Vec<String> {
        match ty {
            Type::Path(ty) => (ty.path.segments.iter())
                .filter_map(|segment| if_then_some!(let syn::PathArguments::AngleBracketed(inner) = &segment.arguments, inner))
                .flat_map(Self::extract_generic_arguments)
                .collect_vec(),
            _ => vec![],
        }
    }

    fn extract_generic_arguments(args: &syn::AngleBracketedGenericArguments) -> Vec<String> {
        (args.args.iter())
            .flat_map(|arg| match arg {
                GenericArgument::Lifetime(inner) => vec![inner.ident.to_string()],
                GenericArgument::Type(inner) => Self::extract(inner),
                _ => vec![],
            })
            .collect_vec()
    }
}

pub(crate) const LIFETIME_STATIC: &str = "static";
