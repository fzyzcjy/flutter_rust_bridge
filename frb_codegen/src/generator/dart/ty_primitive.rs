use crate::generator::dart::gen_wire2api_simple_type_cast;
use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypePrimitiveGenerator, IrTypePrimitive);

impl TypeDartGeneratorTrait for TypePrimitiveGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        match self.ir {
            IrTypePrimitive::I64 | IrTypePrimitive::U64 => Acc {
                io: Some("return raw;".into()),
                wasm: Some("return castNativeBigInt(raw);".into()),
                ..Default::default()
            },
            _ => "return raw;".into(),
        }
    }

    fn wire2api_body(&self) -> String {
        match self.ir {
            IrTypePrimitive::Unit => "return;".to_owned(),
            IrTypePrimitive::I64 | IrTypePrimitive::U64 | IrTypePrimitive::Usize => {
                "return castInt(raw);".to_owned()
            }
            _ => gen_wire2api_simple_type_cast(&self.ir.dart_api_type()),
        }
    }
}
