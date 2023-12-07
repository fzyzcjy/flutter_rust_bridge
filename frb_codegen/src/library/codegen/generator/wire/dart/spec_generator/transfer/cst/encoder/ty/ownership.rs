use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::dart2rust::misc::dart_wire_type_from_rust_wire_type_or_wasm;
use crate::codegen::generator::wire::dart::spec_generator::dart2rust::ty::WireDartGeneratorDart2RustTrait;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::encoder::ty::WireDartTransferCstGeneratorEncoderTrait;

impl<'a> WireDartTransferCstGeneratorEncoderTrait for OwnershipWireDartTransferCstGenerator<'a> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        unreachable!()
    }

    fn dart_wire_type(&self, target: Target) -> String {
        unreachable!()
    }
}
