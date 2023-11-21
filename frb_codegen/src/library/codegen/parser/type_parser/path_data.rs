use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::unencodable::{Args, NameComponent};
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::Primitive;
use crate::codegen::parser::type_parser::ty::TypeParserParsingContext;
use crate::codegen::parser::type_parser::TypeParser;
use crate::if_then_some;
use anyhow::Result;
use anyhow::{anyhow, Context};
use quote::ToTokens;
use syn::{
    AngleBracketedGenericArguments, GenericArgument, ParenthesizedGenericArguments, Path,
    PathArguments, PathSegment,
};

impl<'a> TypeParser<'a> {
    pub(crate) fn extract_path_data(
        &mut self,
        path: &Path,
        context: &TypeParserParsingContext,
    ) -> Result<Vec<NameComponent>> {
        path.segments
            .iter()
            .map(|segment| self.parse_path_segment(path, segment, context))
            .collect()
    }

    fn parse_path_segment(
        &mut self,
        path: &Path,
        segment: &PathSegment,
        context: &TypeParserParsingContext,
    ) -> Result<NameComponent> {
        let ident = segment.ident.to_string();
        let args = match &segment.arguments {
            PathArguments::None => None,
            PathArguments::AngleBracketed(args) => {
                let ir_types = self
                    .parse_angle_bracketed_generic_arguments(args, context)
                    .with_context(|| {
                        anyhow!("\"{ident}\" of \"{}\" is not valid", path.to_token_stream())
                    })?;
                Some(Args::Generic(ir_types))
            }
            PathArguments::Parenthesized(args) => Some(Args::Signature(
                self.parse_parenthesized_generic_arguments(args)?,
            )),
        };
        Ok(NameComponent { ident, args })
    }

    fn parse_angle_bracketed_generic_arguments(
        &mut self,
        args: &AngleBracketedGenericArguments,
        context: &TypeParserParsingContext,
    ) -> Result<Vec<IrType>> {
        args.args
            .iter()
            .filter_map(|arg| if_then_some!(let GenericArgument::Type(ty) = arg, ty))
            .map(|ty| self.parse_type(ty, context))
            .collect()
    }

    fn parse_parenthesized_generic_arguments(
        &mut self,
        args: &ParenthesizedGenericArguments,
        context: &TypeParserParsingContext,
    ) -> Result<Vec<IrType>> {
        let input_types = args
            .inputs
            .iter()
            .map(|ty| self.parse_type(ty, context))
            .collect::<Result<Vec<_>>>()?;

        let output_type = match &args.output {
            syn::ReturnType::Default => Primitive(IrTypePrimitive::Unit),
            syn::ReturnType::Type(_, ret_ty) => self.parse_type(ret_ty)?,
        };

        Ok({
            let mut ans = vec![output_type];
            ans.extend(input_types);
            ans
        })
    }
}
