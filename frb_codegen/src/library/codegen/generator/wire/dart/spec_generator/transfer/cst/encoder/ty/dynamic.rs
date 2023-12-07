use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::encoder::ty::WireDartTransferCstGeneratorEncoderTrait;

impl<'a> WireDartTransferCstGeneratorEncoderTrait for DynamicWireDartTransferCstGenerator<'a> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        Acc::default()
    }

    fn dart_wire_type(&self, _target: Target) -> String {
        // Functions cannot receive dynamic parameters
        "UNREACHABLE_DART_WIRE_TYPE".into()
    }
}
