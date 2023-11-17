use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::wire2api::ty::WireRustGeneratorWire2apiTrait;

impl<'a> WireRustGeneratorWire2apiTrait for RecordWireRustGenerator<'a> {
    fn generate_wire2api_class(&self) -> Option<String> {
        self.as_struct_generator().generate_wire2api_class()
    }
}
