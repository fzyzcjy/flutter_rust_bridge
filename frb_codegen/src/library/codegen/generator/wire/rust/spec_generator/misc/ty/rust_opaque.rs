use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;

impl<'a> WireRustGeneratorMiscTrait for RustOpaqueWireRustGenerator<'a> {
    fn generate_imports(&self) -> Option<Vec<String>> {
        // TODO
        // Some(vec![format!("use {};", path.join("::"))])
        None
    }
}
