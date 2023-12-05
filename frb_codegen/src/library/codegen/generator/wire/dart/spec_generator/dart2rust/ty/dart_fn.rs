use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::dart2rust::ty::WireDartGeneratorDart2RustTrait;
use crate::codegen::ir::ty::dart_opaque::IrTypeDartOpaque;
use crate::codegen::ir::ty::IrType;

impl<'a> WireDartGeneratorDart2RustTrait for DartFnWireDartGenerator<'a> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        Acc::new_common(Some("return api2wire_DartOpaque(raw);".to_owned()))
    }

    fn dart_wire_type(&self, target: Target) -> String {
        WireDartGenerator::new(IrType::DartOpaque(IrTypeDartOpaque), self.context)
            .dart_wire_type(target)
    }
}
