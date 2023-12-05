use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::unencodable::IrTypeUnencodable;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::Primitive;
use crate::codegen::parser::type_parser::unencodable::{ArgsRefs, SplayedSegment};
use crate::codegen::parser::type_parser::TypeParserWithContext;
use ArgsRefs::Generic;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path_data_dart_fn(
        &mut self,
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<IrType>> {
        Ok(Some(match last_segment {
            ("DartFn", Some(Generic([IrType::Unencodable(IrTypeUnencodable { string, .. })]))) => {
                self.parse_dart_fn(string)
            }

            _ => return Ok(None),
        }))
    }

    fn parse_dart_fn(&mut self, raw: &str) -> IrType {
        todo!("{}", raw)
    }
}

// Use this unit "test" to see how a type will be parsed into a tree
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_syn_parse_example() {
//         let ans: syn::Type = syn::parse_str("fn(DartOpaque) -> BoxFuture<String>").unwrap();
//         println!("{ans:#?}");
//     }
// }
