use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::Primitive;
use crate::codegen::parser::type_parser::unencodable::SplayedSegment;
use crate::codegen::parser::type_parser::TypeParserWithContext;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path_data_primitive(
        &mut self,
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<IrType>> {
        Ok(Some(match last_segment {
            // TODO: change to "if let guard" https://github.com/rust-lang/rust/issues/51114
            (name, None) if matches!(parse_primitive(name), Some(..)) => {
                Primitive(parse_primitive(name).unwrap())
            }

            _ => return Ok(None),
        }))
    }
}

fn parse_primitive(s: &str) -> Option<IrTypePrimitive> {
    Some(match s {
        "u8" => IrTypePrimitive::U8,
        "i8" => IrTypePrimitive::I8,
        "u16" => IrTypePrimitive::U16,
        "i16" => IrTypePrimitive::I16,
        "u32" => IrTypePrimitive::U32,
        "i32" => IrTypePrimitive::I32,
        "u64" => IrTypePrimitive::U64,
        "i64" => IrTypePrimitive::I64,
        "f32" => IrTypePrimitive::F32,
        "f64" => IrTypePrimitive::F64,
        "bool" => IrTypePrimitive::Bool,
        "()" => IrTypePrimitive::Unit,
        "usize" => IrTypePrimitive::Usize,
        "isize" => IrTypePrimitive::Isize,
        _ => return None,
    })
}
