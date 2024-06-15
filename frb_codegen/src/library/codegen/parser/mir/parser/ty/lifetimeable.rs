use crate::codegen::ir::mir::ty::rust_auto_opaque_implicit::MirTypeRustAutoOpaqueImplicit;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::parser::ty::TypeParserWithContext;
use crate::if_then_some;
use itertools::Itertools;
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

fn extract_lifetimes(ty: &Type) -> Vec<String> {
    match ty {
        Type::Path(ty) => (ty.path.segments.iter())
            .filter_map(|segment| if_then_some!(let syn::PathArguments::AngleBracketed(inner) = &segment.arguments, inner))
            .flat_map(extract_generic_arguments_lifetimes)
            .collect_vec(),
        _ => vec![],
    }
}

fn extract_generic_arguments_lifetimes(
    args: &syn::AngleBracketedGenericArguments,
) -> Vec<String> {
    (args.args.iter())
        .filter_map(|arg| if_then_some!(let GenericArgument::Lifetime(inner) = arg, inner))
        .map(|lifetime| lifetime.ident.to_string())
        .collect_vec()
}
