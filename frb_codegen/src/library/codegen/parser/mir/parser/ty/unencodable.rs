use crate::codegen::ir::mir::ty::rust_opaque::NameComponent;
use syn::Type;

pub(crate) type SplayedSegment<'a> = (&'a str, &'a [Type]);

/// Spread and turn out the data of a fully qualified name for structural pattern matching.
pub(crate) fn splay_segments(segments: &[NameComponent]) -> Vec<SplayedSegment<'_>> {
    segments
        .iter()
        .map(|NameComponent { ident, args }| (&ident[..], &args[..]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::splay_segments;
    use crate::codegen::ir::mir::ty::rust_opaque::NameComponent;
    use syn::parse_quote;

    /// Preserves qualified segment names and their generic arguments.
    #[test]
    fn splay_segments_preserves_names_and_arguments() {
        let segments = vec![
            NameComponent {
                ident: "crate".into(),
                args: vec![],
            },
            NameComponent {
                ident: "Container".into(),
                args: vec![parse_quote!(String), parse_quote!(Vec<u8>)],
            },
        ];

        let actual = splay_segments(&segments);

        assert_eq!(actual.len(), 2);
        assert_eq!(actual[0].0, "crate");
        assert!(actual[0].1.is_empty());
        assert_eq!(actual[1].0, "Container");
        assert_eq!(actual[1].1, &[parse_quote!(String), parse_quote!(Vec<u8>)]);
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
