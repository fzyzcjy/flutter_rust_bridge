use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for PrimitiveCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> String {
        match self.ir {
            IrTypePrimitive::Unit => "".into(),
            _ => format!(
                "serializer.buffer.put{}(self);",
                get_serializer_postfix(&self.ir)
            ),
        }
    }

    fn generate_decode(&self, lang: &Lang) -> String {
        format!("return TODO_depend_on_serializer;")
    }
}

fn get_serializer_postfix(prim: &IrTypePrimitive) -> &'static str {
    match prim {
        IrTypePrimitive::U8 => "Uint8",
        IrTypePrimitive::I8 => "Int8",
        IrTypePrimitive::U16 => "Uint16",
        IrTypePrimitive::I16 => "Int16",
        IrTypePrimitive::U32 => "Uint32",
        IrTypePrimitive::I32 => "Int32",
        IrTypePrimitive::U64 => "Uint64",
        IrTypePrimitive::I64 => "Int64",
        IrTypePrimitive::Usize => "Uint64",
        IrTypePrimitive::Isize => "Int64",
        IrTypePrimitive::F32 => "Float32",
        IrTypePrimitive::F64 => "Float64",
        IrTypePrimitive::Bool => "Uint8",
        IrTypePrimitive::Unit => unreachable!(),
    }
}
