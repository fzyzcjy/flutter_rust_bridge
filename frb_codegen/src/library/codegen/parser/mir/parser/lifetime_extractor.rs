use crate::if_then_some;
use itertools::{concat, Itertools};
use syn::{GenericArgument, Type};

pub(crate) struct LifetimeExtractor;

impl LifetimeExtractor {
    pub(crate) fn extract_skipping_static(ty: &Type) -> Vec<Lifetime> {
        // log::debug!("extract_skipping_static ans={ans:?} ty={ty:?}");
        (Self::extract(ty).into_iter())
            .filter(|lifetime| !lifetime.is_static())
            .collect_vec()
    }

    fn extract(ty: &Type) -> Vec<Lifetime> {
        match ty {
            Type::Path(ty) => (ty.path.segments.iter())
                .filter_map(|segment| if_then_some!(let syn::PathArguments::AngleBracketed(inner) = &segment.arguments, inner))
                .flat_map(Self::extract_generic_arguments)
                .collect_vec(),
            Type::Reference(ty) => concat([
                ty.lifetime.iter().map(|x| x.into()).collect(),
                Self::extract(&ty.elem),
            ]),
            _ => vec![],
        }
    }

    fn extract_generic_arguments(args: &syn::AngleBracketedGenericArguments) -> Vec<Lifetime> {
        (args.args.iter())
            .flat_map(|arg| match arg {
                GenericArgument::Lifetime(inner) => vec![inner.into()],
                GenericArgument::Type(inner) => Self::extract(inner),
                _ => vec![],
            })
            .collect_vec()
    }
}

pub(crate) const LIFETIME_STATIC: &str = "static";
pub(crate) const LIFETIME_ANONYMOUS: &str = "_";

// TODO maybe move
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Lifetime(pub String);

impl Lifetime {
    pub(crate) fn is_static(&self) -> bool {
        self.0 == LIFETIME_STATIC
    }

    pub(crate) fn is_anonymous(&self) -> bool {
        self.0 == LIFETIME_ANONYMOUS
    }
}

impl From<&syn::Lifetime> for Lifetime {
    fn from(value: &syn::Lifetime) -> Self {
        Self(value.ident.to_string())
    }
}
