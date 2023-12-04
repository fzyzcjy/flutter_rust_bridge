use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;

impl<'a> WireRustGeneratorMiscTrait for RecordWireRustGenerator<'a> {}

impl RecordWireRustGenerator<'_> {
    pub(crate) fn as_struct_generator(&self) -> StructRefWireRustGenerator {
        StructRefWireRustGenerator {
            ir: self.ir.inner.clone(),
            context: self.context,
        }
    }
}
