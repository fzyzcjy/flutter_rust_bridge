use crate::codegen::generator::wire::rust::spec_generator::api2wire::ty::WireRustGeneratorApi2wireTrait;
use crate::codegen::generator::wire::rust::spec_generator::base::*;

impl<'a> WireRustGeneratorApi2wireTrait for BoxedWireRustGenerator<'a> {
    fn generate_access_object_core(&self, obj: String) -> String {
        format!("(*{obj})")
    }
}
