use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::class::misc::generate_class_from_fields;
use crate::codegen::generator::wire::rust::class::ty::WireRustClassGeneratorClassTrait;

impl<'a> WireRustClassGeneratorClassTrait for PrimitiveListWireRustGenerator<'a> {
    fn generate_class(&self) -> Option<String> {
        Some(generate_class_from_fields(
            self.ir.clone(),
            &self.context,
            &vec![
                format!("ptr: *mut {}", self.ir.primitive.rust_wire_type(Target::Io)),
                "len: i32".to_string(),
            ],
        ))
    }
}
