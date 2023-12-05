use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::{Target, TargetOrCommon};
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::dart2rust::ty::WireDartGeneratorDart2RustTrait;

impl<'a> WireDartGeneratorDart2RustTrait for DartOpaqueWireDartGenerator<'a> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        Acc::new(|target| match target {
            TargetOrCommon::Io | TargetOrCommon::Wasm => {
                Some("return api2wireDartOpaque(raw);".to_owned())
            }
            TargetOrCommon::Common => None,
        })
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        Some(
            "wireObj.handle = generalizedFrbRustBinding.newDartOpaque(apiObj);
            wireObj.port = portManager.dartOpaqueDropPort;"
                .to_owned(),
        )
    }

    fn dart_wire_type(&self, target: Target) -> String {
        match target {
            Target::Io => "wire_DartOpaque",
            Target::Wasm => "Object",
        }
        .to_owned()
    }
}
