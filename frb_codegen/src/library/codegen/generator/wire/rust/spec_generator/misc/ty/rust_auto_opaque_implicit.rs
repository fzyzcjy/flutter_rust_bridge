use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;
use crate::utils::namespace::Namespace;

impl WireRustGeneratorMiscTrait for RustAutoOpaqueImplicitWireRustGenerator<'_> {
    fn generate_imports(&self) -> Option<Vec<Namespace>> {
        Some(vec![self.mir.inner.namespace.clone()])
    }
}
