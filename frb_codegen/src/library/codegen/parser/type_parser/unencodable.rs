use crate::codegen::ir::ty::rust_opaque::NameComponent;
use crate::codegen::ir::ty::IrType;
use quote::ToTokens;
use syn::TypePath;

pub(crate) type SplayedSegment<'a> = (&'a str, &'a [IrType]);

/// Spread and turn out the data of a fully qualified name for structural pattern matching.
pub(crate) fn splay_segments(segments: &[NameComponent]) -> Vec<SplayedSegment> {
    segments
        .iter()
        .map(|NameComponent { ident, args }| (&ident[..], &args[..]))
        .collect()
}

// TODO
// pub(crate) fn parse_path_type_to_unencodable(
//     type_path: &TypePath,
//     splayed_segments: &[SplayedSegment],
// ) -> IrType {
//     Unencodable(IrTypeUnencodable {
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
