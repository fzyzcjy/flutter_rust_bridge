use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::common::ty::WireRustGeneratorCommonTrait;

impl<'a> WireRustGeneratorCommonTrait for OptionalWireRustGenerator<'a> {
    fn generate_imports(&self) -> Option<Vec<String>> {
        generate_import(&self.ir.inner, self.context.ir_pack, self.context.config)
    }
}
