use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::Primitive;
use crate::codegen::parser::type_parser::unencodable::{ArgsRefs, SplayedSegment};
use crate::codegen::parser::type_parser::TypeParserWithContext;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path_data_dart_fn(
        &mut self,
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<IrType>> {
        Ok(Some(match last_segment {
            ("DartFn", inner) => self.parse_dart_fn(inner),

            _ => return Ok(None),
        }))
    }

    fn parse_dart_fn(&mut self, inner: &Option<ArgsRefs>) -> IrType {
        log::info!("hi {inner:?}");
        todo!()
    }
}
