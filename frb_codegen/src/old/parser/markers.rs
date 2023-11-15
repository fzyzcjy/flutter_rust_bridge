use syn::*;

// TODO use the `metadata_parser` instead
// /// Extract a path from marker `#[frb(mirror(path), ..)]`
// pub fn extract_mirror_marker(attrs: &[Attribute]) -> Vec<Path> {
//     let mut paths = vec![];
//     attrs
//         .iter()
//         .filter(|attr| attr.path().is_ident("frb"))
//         .for_each(|attr| {
//             let _ = attr.parse_nested_meta(|meta| {
//                 if meta.path.is_ident("mirror") {
//                     meta.parse_nested_meta(|path| {
//                         paths.push(path.path);
//                         Ok(())
//                     })
//                     .unwrap();
//                 }
//                 Ok(())
//             });
//         });
//     paths
// }

// TODO use the `metadata_parser` instead
// /// Checks if the `#[frb(non_final)]` attribute is present.
// pub fn has_non_final(attrs: &[Attribute]) -> bool {
//     attrs
//         .iter()
//         .filter(|attr| attr.path().is_ident("frb"))
//         .any(|attr| {
//             let mut flag = false;
//             let _ = attr.parse_nested_meta(|arg| {
//                 if arg.path.is_ident("non_final") {
//                     flag = true;
//                 }
//                 Ok(())
//             });
//             flag
//         })
// }
