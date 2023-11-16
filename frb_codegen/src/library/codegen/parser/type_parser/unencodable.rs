use crate::codegen::ir::ty::unencodable::{Args, IrTypeUnencodable, NameComponent};
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::Unencodable;
use quote::ToTokens;
use syn::TypePath;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub(crate) enum ArgsRefs<'a> {
    Generic(&'a [IrType]),
    Signature(&'a [IrType]),
}

pub(crate) trait Splayable {
    fn splay(&self) -> Vec<(&str, Option<ArgsRefs>)>;
}

impl Splayable for Vec<NameComponent> {
    /// Spread and turn out the data of a fully qualified name for structural pattern matching.
    fn splay(&self) -> Vec<(&str, Option<ArgsRefs>)> {
        return self
            .iter()
            .map(|NameComponent { ident, args }| {
                (
                    &ident[..],
                    args.as_ref().map(|args| match &args {
                        Args::Generic(types) => ArgsRefs::Generic(&types[..]),
                        Args::Signature(types) => ArgsRefs::Signature(&types[..]),
                    }),
                )
            })
            .collect();
    }
}

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
