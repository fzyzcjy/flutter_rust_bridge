use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::api_dart::base::ApiDartGenerator;
use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::dart::spec_generator::api2wire::ty::WireDartGeneratorApi2wireTrait;
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;

impl<'a> WireDartGeneratorApi2wireTrait for PrimitiveWireDartGenerator<'a> {
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

    fn dart_wire_type(&self, target: Target) -> String {
        match &self.ir {
            IrTypePrimitive::I64 | IrTypePrimitive::U64 if target == Target::Wasm => {
                "Object".into()
            }
            _ => ApiDartGenerator::new(self.ir.clone(), self.context.as_api_dart_context())
                .dart_api_type(),
        }
    }
}
