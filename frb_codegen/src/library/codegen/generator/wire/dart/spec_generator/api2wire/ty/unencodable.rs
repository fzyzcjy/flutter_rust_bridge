use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::dart::spec_generator::api2wire::ty::WireDartGeneratorApi2wireTrait;
use crate::codegen::generator::wire::dart::spec_generator::base::*;

impl<'a> WireDartGeneratorApi2wireTrait for UnencodableWireDartGenerator<'a> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        unreachable!()
    }

    fn dart_wire_type(&self, target: Target) -> String {
        match target {
            Target::Io => self.rust_wire_type(target),
            Target::Wasm => "Object".into(),
        }
    }
}
