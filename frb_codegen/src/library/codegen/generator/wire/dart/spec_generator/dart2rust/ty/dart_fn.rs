use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::dart2rust::ty::WireDartGeneratorDart2RustTrait;

impl<'a> WireDartGeneratorDart2RustTrait for DartFnWireDartGenerator<'a> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        Acc {
            io: Some("todo_api2wire_body;".into()),
            wasm: Some("todo_api2wire_body;".into()),
            ..Default::default()
        }
    }

    fn dart_wire_type(&self, target: Target) -> String {
        "todo_dart_wire_type".into()
    }
}
