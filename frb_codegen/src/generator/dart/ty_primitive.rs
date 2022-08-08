use crate::generator::dart::gen_wire2api_simple_type_cast;
use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypePrimitiveGenerator, IrTypePrimitive);

impl TypeDartGeneratorTrait for TypePrimitiveGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        "return raw;".into()
    }

    fn wire2api_body(&self) -> String {
        match self.ir {
            IrTypePrimitive::Unit => "return;".to_owned(),
            _ => gen_wire2api_simple_type_cast(&self.ir.dart_api_type()),
        }
    }
}
