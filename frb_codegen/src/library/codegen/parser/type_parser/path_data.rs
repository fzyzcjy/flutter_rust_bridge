use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::unencodable::{Args, NameComponent};
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::Primitive;
use crate::codegen::parser::type_parser::TypeParser;
use quote::ToTokens;
use syn::{
    AngleBracketedGenericArguments, GenericArgument, ParenthesizedGenericArguments, Path,
    PathArguments,
};

impl<'a> TypeParser<'a> {
    fn angle_bracketed_generic_arguments_to_ir_types(
        &mut self,
        args: &AngleBracketedGenericArguments,
    ) -> std::result::Result<Vec<IrType>, String> {
        let AngleBracketedGenericArguments { args, .. } = args;
        args.iter()
            .filter_map(|arg| {
                if let GenericArgument::Type(ty) = arg {
                    Some(Ok(self.parse_type(ty)))
                } else {
                    None
                }
            })
            .collect()
    }

    fn parenthesized_generic_arguments_to_ir_types(
        &mut self,
        args: &ParenthesizedGenericArguments,
    ) -> Vec<IrType> {
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
                args.insert(0, self.parse_type(ret_ty));
            }
        };

        args
    }

    fn extract_path_data(
        &mut self,
        path: &Path,
    ) -> std::result::Result<Vec<NameComponent>, String> {
        let Path { segments, .. } = path;

        let data: std::result::Result<Vec<NameComponent>, String> = segments
            .iter()
            .map(|segment| {
                let ident = segment.ident.to_string();
                match &segment.arguments {
                    PathArguments::None => Ok(NameComponent { ident, args: None }),
                    PathArguments::AngleBracketed(args) => {
                        match self.angle_bracketed_generic_arguments_to_ir_types(args) {
                            Err(sub_err) => Err(format!(
                                "\"{}\" of \"{}\" is not valid. {}",
                                ident,
                                path.to_token_stream(),
                                sub_err
                            )),
                            Ok(ir_types) => Ok(NameComponent {
                                ident,
                                args: Some(Args::Generic(ir_types)),
                            }),
                        }
                    }
                    PathArguments::Parenthesized(args) => Ok(NameComponent {
                        ident,
                        args: Some(Args::Signature(
                            self.parenthesized_generic_arguments_to_ir_types(args),
                        )),
                    }),
                }
            })
            .collect();

        let storage = data?;

        Ok(storage)
    }
}
