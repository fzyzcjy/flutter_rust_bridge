use crate::codegen::ir::ty::unencodable::{Args, IrTypeUnencodable, NameComponent};
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::Unencodable;
use crate::codegen::parser::unencodable::ArgsRefs;
use quote::ToTokens;
use syn::TypePath;

pub(crate) fn parse_path_type_to_unencodable(
    type_path: &TypePath,
    flat_vector: Vec<(&str, Option<ArgsRefs>)>,
) -> IrType {
    Unencodable(IrTypeUnencodable {
        string: type_path.to_token_stream().to_string(),
        segments: flat_vector
            .iter()
            .map(|(ident, option_args_refs)| NameComponent {
                ident: ident.to_string(),
                args: option_args_refs.as_ref().map(|args_refs| match args_refs {
                    ArgsRefs::Generic(args_array) => Args::Generic(args_array.to_vec()),
                    ArgsRefs::Signature(args_array) => Args::Signature(args_array.to_vec()),
                }),
            })
            .collect(),
    })
}
