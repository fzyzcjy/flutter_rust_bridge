use crate::{codegen::ir::mir::ty::rust_opaque::NameComponent, if_then_some};
use syn::{AssocType, GenericArgument, Type};

#[derive(Clone, Debug)]
pub(crate) struct SplayedSegment<'a> {
    pub name: &'a str,
    pub args: &'a [GenericArgument],
}

/// Spread and turn out the data of a fully qualified name for structural pattern matching.
pub(crate) fn splay_segments(segments: &[NameComponent]) -> Vec<SplayedSegment> {
    segments
        .iter()
        .map(|NameComponent { ident, args }| SplayedSegment::new(&ident[..], &args[..]))
        .collect()
}

impl<'a> SplayedSegment<'a> {
    pub(crate) fn new(name: &'a str, args: &'a [GenericArgument]) -> Self {
        Self { name, args }
    }

    pub(crate) fn type_arguments(&self) -> Vec<Type> {
        self.args
            .iter()
            .filter_map(|arg| if_then_some!(let GenericArgument::Type(ty) = arg, ty.clone()))
            .collect()
    }

    pub(crate) fn associated_type_arguments(&self) -> Vec<AssocType> {
        self.args
            .iter()
            .filter_map(|arg| if_then_some!(let GenericArgument::AssocType(ty) = arg, ty.clone()))
            .collect()
    }
}

impl<'a> Into<(&'a str, Vec<Type>)> for SplayedSegment<'a> {
    fn into(self) -> (&'a str, Vec<Type>) {
        (self.name, self.type_arguments())
    }
}

impl<'a> Into<(&'a str, Vec<AssocType>)> for SplayedSegment<'a> {
    fn into(self) -> (&'a str, Vec<AssocType>) {
        (self.name, self.associated_type_arguments())
    }
}

impl<'a> Into<(&'a str, Vec<Type>, Vec<AssocType>)> for SplayedSegment<'a> {
    fn into(self) -> (&'a str, Vec<Type>, Vec<AssocType>) {
        (
            self.name,
            self.type_arguments(),
            self.associated_type_arguments(),
        )
    }
}

// TODO
// pub(crate) fn parse_path_type_to_unencodable(
//     type_path: &TypePath,
//     splayed_segments: &[SplayedSegment],
// ) -> MirType {
//     Unencodable(MirTypeUnencodable {
//         namespace: None,
//         string: type_path.to_token_stream().to_string(),
//         segments: splayed_segments
//             .iter()
//             .map(|(ident, option_args_refs)| NameComponent {
//                 ident: ident.to_string(),
//                 args: option_args_refs.as_ref().map(|args_refs| match args_refs {
//                     ArgsRefs::Generic(args_array) => Args::Generic(args_array.to_vec()),
//                     ArgsRefs::Signature(args_array) => Args::Signature(args_array.to_vec()),
//                 }),
//             })
//             .collect(),
//     })
// }
