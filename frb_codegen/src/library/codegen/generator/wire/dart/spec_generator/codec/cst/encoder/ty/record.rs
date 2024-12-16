use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::struct_or_record::StructOrRecord::Record;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::misc::dart_wire_type_from_rust_wire_type_or_web;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::structure::GeneralizedStructGenerator;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;

impl<'a> WireDartCodecCstGeneratorEncoderTrait for RecordWireDartCodecCstGenerator<'a> {
    fn generate_encode_func_body(&self) -> Acc<Option<String>> {
        self.new_generalized_generator().generate_encode_func_body()
    }

    fn generate_encode_api_fill_to_wire_body(&self) -> Option<String> {
        self.new_generalized_generator().api_fill_to_wire_body()
    }

    fn dart_wire_type(&self, target: Target) -> String {
        dart_wire_type_from_rust_wire_type_or_web(self, target, "JSAny".into())
    }
}

impl<'a> RecordWireDartCodecCstGenerator<'a> {
    fn new_generalized_generator(&self) -> GeneralizedStructGenerator {
        GeneralizedStructGenerator::new(self.mir.inner.clone(), self.context, Record)
    }
}
