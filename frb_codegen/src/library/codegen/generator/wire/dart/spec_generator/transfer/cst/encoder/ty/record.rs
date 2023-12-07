use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::dart2rust::misc::dart_wire_type_from_rust_wire_type_or_wasm;
use crate::codegen::generator::wire::dart::spec_generator::dart2rust::ty::structure::{
    GeneralizedStructGenerator, GeneralizedStructGeneratorMode,
};
use crate::codegen::generator::wire::dart::spec_generator::dart2rust::ty::WireDartGeneratorDart2RustTrait;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::encoder::ty::WireDartTransferCstGeneratorEncoderTrait;
use GeneralizedStructGeneratorMode::Record;

impl<'a> WireDartTransferCstGeneratorEncoderTrait for RecordWireDartTransferCstGenerator<'a> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        self.new_generalized_generator().api2wire_body()
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        self.new_generalized_generator().api_fill_to_wire_body()
    }

    fn dart_wire_type(&self, target: Target) -> String {
        dart_wire_type_from_rust_wire_type_or_wasm(self, target, "List<dynamic>".into())
    }
}

impl<'a> RecordWireDartTransferCstGenerator<'a> {
    fn new_generalized_generator(&self) -> GeneralizedStructGenerator {
        GeneralizedStructGenerator::new(self.ir.inner.clone(), self.context, Record)
    }
}
