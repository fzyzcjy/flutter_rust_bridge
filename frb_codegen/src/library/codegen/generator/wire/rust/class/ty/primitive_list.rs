use crate::codegen::generator::api_dart::base::PrimitiveListApiDartGenerator;
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::class::ty::WireRustClassGeneratorClassTrait;

impl<'a> WireRustClassGeneratorClassTrait for PrimitiveListApiDartGenerator<'a> {
    fn generate_class(&self) -> Option<String> {
        Some(vec![
            format!("ptr: *mut {}", self.ir.primitive.rust_wire_type(Target::Io)),
            "len: i32".to_string(),
        ])
    }
}
