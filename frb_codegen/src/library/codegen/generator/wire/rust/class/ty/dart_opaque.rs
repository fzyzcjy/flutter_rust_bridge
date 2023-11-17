use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::class::ty::WireRustClassGeneratorClassTrait;

impl<'a> WireRustClassGeneratorClassTrait for DartOpaqueWireRustGenerator<'a> {
    fn generate_class(&self) -> Option<String> {
        Some(vec!["port: i64".to_owned(), "handle: usize".to_owned()])
    }
}
