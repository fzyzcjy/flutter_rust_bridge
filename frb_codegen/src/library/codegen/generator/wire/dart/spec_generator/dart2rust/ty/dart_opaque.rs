use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::{Target, TargetOrCommon};
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::dart2rust::ty::rust_opaque::dart_or_rust_opaque_dart_wire_type;
use crate::codegen::generator::wire::dart::spec_generator::dart2rust::ty::WireDartGeneratorDart2RustTrait;

impl<'a> WireDartGeneratorDart2RustTrait for DartOpaqueWireDartGenerator<'a> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        Acc::new(|target| match target {
            TargetOrCommon::Io => {
                Some("return wire.dart_opaque_dart2rust_api2wire(raw);".to_owned())
            }
            TargetOrCommon::Wasm => Some("return raw;".to_owned()),
            TargetOrCommon::Common => None,
        })
    }

    fn dart_wire_type(&self, target: Target) -> String {
        dart_or_rust_opaque_dart_wire_type(target)
    }
}
