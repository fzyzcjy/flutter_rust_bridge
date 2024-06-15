use crate::codegen::ir::mir::ty::rust_auto_opaque_implicit::MirTypeRustAutoOpaqueImplicit;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::parser::ty::TypeParserWithContext;
use crate::if_then_some;
use itertools::Itertools;
use syn::fold::fold_lifetime;
use syn::{GenericArgument, Type};

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_maybe_lifetimeable(
        &mut self,
        original: MirTypeRustAutoOpaqueImplicit,
    ) -> anyhow::Result<MirType> {
        Ok(self
            .parse_maybe_lifetimeable_raw(&original.raw.string)?
            .unwrap_or(MirType::RustAutoOpaqueImplicit(original)))
    }

    fn parse_maybe_lifetimeable_raw(&self, original: &str) -> anyhow::Result<Option<MirType>> {
        let ty: Type = syn::parse_str(original)?;
        Ok(TODO)
    }
}

struct LifetimeExtractor;

impl LifetimeExtractor {
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
