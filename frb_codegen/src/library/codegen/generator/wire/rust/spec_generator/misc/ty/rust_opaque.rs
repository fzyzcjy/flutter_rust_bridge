use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;
use crate::codegen::ir::namespace::Namespace;

impl<'a> WireRustGeneratorMiscTrait for RustOpaqueWireRustGenerator<'a> {
    fn generate_imports(&self) -> Option<Vec<String>> {
        // To expose the `pub use`s inside that file
        Some(vec![format!("use {}::*;", self.ir.namespace.joined_path)])
    }
}
