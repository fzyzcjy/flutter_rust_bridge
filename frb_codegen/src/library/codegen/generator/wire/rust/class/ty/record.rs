use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::class::ty::WireRustClassGeneratorClassTrait;

impl<'a> WireRustClassGeneratorClassTrait for RecordWireRustGenerator<'a> {
    fn generate_class(&self) -> Option<String> {
        self.as_struct_generator().generate_class()
    }
}
