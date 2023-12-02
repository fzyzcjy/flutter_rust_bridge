use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::dart2rust::misc::dart_wire_type_from_rust_wire_type_or_wasm;
use crate::codegen::generator::wire::dart::spec_generator::dart2rust::ty::WireDartGeneratorApi2wireTrait;

impl<'a> WireDartGeneratorApi2wireTrait for UnencodableWireDartGenerator<'a> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        unreachable!()
    }

    fn dart_wire_type(&self, target: Target) -> String {
        dart_wire_type_from_rust_wire_type_or_wasm(self, target, "Object".into())
    }
}
