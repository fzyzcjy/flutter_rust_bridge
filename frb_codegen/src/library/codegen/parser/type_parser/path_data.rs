use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::unencodable::{Args, NameComponent};
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::Primitive;
use crate::codegen::parser::type_parser::TypeParser;
use anyhow::bail;
use quote::ToTokens;
use syn::{
    AngleBracketedGenericArguments, GenericArgument, ParenthesizedGenericArguments, Path,
    PathArguments, PathSegment,
};

impl<'a> TypeParser<'a> {
    fn extract_path_data(&mut self, path: &Path) -> anyhow::Result<Vec<NameComponent>> {
        let Path { segments, .. } = path;

        segments
            .iter()
            .map(|segment| self.parse_path_segment(path, segment))
            .collect()
    }

    fn parse_path_segment(
        &mut self,
        path: &Path,
        segment: &PathSegment,
    ) -> anyhow::Result<NameComponent, String> {
        let ident = segment.ident.to_string();
        Ok(match &segment.arguments {
            PathArguments::None => NameComponent { ident, args: None },
            PathArguments::AngleBracketed(args) => {
                match self.parse_angle_bracketed_generic_arguments(args) {
                    Err(sub_err) => bail!(
                        "\"{}\" of \"{}\" is not valid. {}",
                        ident,
                        path.to_token_stream(),
                        sub_err
                    ),
                    Ok(ir_types) => NameComponent {
                        ident,
                        args: Some(Args::Generic(ir_types)),
                    },
                }
            }
            PathArguments::Parenthesized(args) => NameComponent {
                ident,
                args: Some(Args::Signature(
                    self.parse_parenthesized_generic_arguments(args)?,
                )),
            },
        })
    }

    fn parse_angle_bracketed_generic_arguments(
        &mut self,
        args: &AngleBracketedGenericArguments,
    ) -> anyhow::Result<Vec<IrType>> {
        let AngleBracketedGenericArguments { args, .. } = args;
        args.iter()
            .filter_map(|arg| {
                if let GenericArgument::Type(ty) = arg {
                    Some(self.parse_type(ty))
                } else {
                    None
                }
            })
            .collect()
    }

    fn parse_parenthesized_generic_arguments(
        &mut self,
        args: &ParenthesizedGenericArguments,
    ) -> anyhow::Result<Vec<IrType>> {
        let ParenthesizedGenericArguments { inputs, output, .. } = args;

        let mut args = inputs
            .iter()
            .map(|ty| self.parse_type(ty))
            .collect::<Vec<IrType>>();

        match output {
            syn::ReturnType::Default => {
                args.insert(0, Primitive(IrTypePrimitive::Unit));
            }
            syn::ReturnType::Type(_, ret_ty) => {
                args.insert(0, self.parse_type(ret_ty)?);
            }
        };

        Ok(args)
    }
}
