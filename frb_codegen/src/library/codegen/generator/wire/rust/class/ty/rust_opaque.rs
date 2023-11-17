use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::class::ty::WireRustClassGeneratorClassTrait;

impl<'a> WireRustClassGeneratorClassTrait for RustOpaqueWireRustGenerator<'a> {
    fn generate_class(&self) -> Option<String> {
        Some(vec!["ptr: *const core::ffi::c_void".to_owned()])
    }
}
