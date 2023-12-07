use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::{Target, TargetOrCommon};
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::encoder::ty::rust_opaque::dart_or_rust_opaque_dart_wire_type;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::encoder::ty::WireDartTransferCstGeneratorEncoderTrait;

impl<'a> WireDartTransferCstGeneratorEncoderTrait for DartOpaqueWireDartTransferCstGenerator<'a> {
    fn encode_func_body(&self) -> Acc<Option<String>> {
        Acc::new(|target| match target {
            TargetOrCommon::Io => {
                Some("return wire.dart_opaque_dart2rust_cst_encode(raw);".to_owned())
            }
            TargetOrCommon::Wasm => Some("return raw;".to_owned()),
            TargetOrCommon::Common => None,
        })
    }

    fn dart_wire_type(&self, target: Target) -> String {
        dart_or_rust_opaque_dart_wire_type(target)
    }
}
