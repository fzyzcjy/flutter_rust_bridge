// use anyhow::Result;
// use syn::{parse_str, visit_mut, visit_mut::VisitMut, Path, Type};
//
// pub(crate) fn parse_type(mut ty: Type) -> Result<Type> {
//     struct Visitor;
//     impl VisitMut for Visitor {
//         fn visit_path_mut(&mut self, node: &mut Path) {
//             if node.segments.len() == 1 {
//                 let ident = &node.segments[0].ident;
//                 if let Some(reconstructed_name) = parse_name(&ident.to_string()).unwrap() {
//                     // println!("hi {node:?} {reconstructed_name}");
//                     *node = parse_str(&reconstructed_name).unwrap();
//                 }
//             }
//
//             visit_mut::visit_path_mut(self, node);
//         }
//     }
//     Visitor.visit_type_mut(&mut ty);
//     Ok(ty)
// }
//
// pub(crate) fn parse_name_or_original(raw_name: &str) -> Result<String> {
//     Ok(parse_name(raw_name)?.unwrap_or_else(|| raw_name.to_string()))
// }
//
// fn parse_name(raw_name: &str) -> Result<Option<String>> {
//     const DUMMY_STRUCT_PREFIX: &str = "__external_impl__";
//     Ok(
//         if let Some(stripped_name) = raw_name.strip_prefix(DUMMY_STRUCT_PREFIX) {
//             Some(String::from_utf8(hex::decode(stripped_name)?)?)
//         } else {
//             None
//         },
//     )
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_parse_external_impl_dummy_struct_name() {
//         assert_eq!(parse_name("One<Two,Three>").unwrap(), None);
//         assert_eq!(
//             parse_name("__external_impl__4f6e65203c2054776f2c205468726565203e").unwrap(),
//             Some("One < Two, Three >".to_owned()),
//         );
//     }
// }
