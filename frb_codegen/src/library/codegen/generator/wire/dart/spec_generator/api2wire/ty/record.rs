use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::dart::spec_generator::api2wire::ty::structure::{
    GeneralizedStructGenerator, GeneralizedStructGeneratorMode,
};
use crate::codegen::generator::wire::dart::spec_generator::api2wire::ty::WireDartGeneratorApi2wireTrait;
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;
use GeneralizedStructGeneratorMode::Record;

impl<'a> WireDartGeneratorApi2wireTrait for RecordWireDartGenerator<'a> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        self.new_generalized_generator().api2wire_body()
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        self.new_generalized_generator().api_fill_to_wire_body()
    }

    fn dart_wire_type(&self, target: Target) -> String {
        if target == Target::Wasm {
            "List<dynamic>".to_string()
        } else {
            self.rust_wire_type(target)
        }
    }
}

impl<'a> RecordWireDartGenerator<'a> {
    fn new_generalized_generator(&self) -> GeneralizedStructGenerator {
        GeneralizedStructGenerator::new(self.ir.inner.clone(), self.context, Record)
    }
}
