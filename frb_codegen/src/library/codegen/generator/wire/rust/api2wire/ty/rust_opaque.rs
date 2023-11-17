use crate::codegen::generator::wire::rust::api2wire::ty::WireRustGeneratorApi2wireTrait;
use crate::codegen::generator::wire::rust::base::*;

impl<'a> WireRustGeneratorApi2wireTrait for RustOpaqueWireRustGenerator<'a> {
    fn generate_convert_to_dart(&self, obj: String) -> String {
        format!("{obj}.into_dart()")
    }
}
