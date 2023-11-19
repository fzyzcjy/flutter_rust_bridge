use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::misc::misc::rust_wire_type_add_prefix_or_js_value;
use crate::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;

impl<'a> WireRustGeneratorMiscTrait for OptionalListWireRustGenerator<'a> {
    // TODO rm this, since we will visit all sub-types to generate
    // fn generate_imports(&self) -> Option<Vec<String>> {
    //     generate_import(&self.ir.inner, self.context.ir_pack, self.context.config)
    // }
}
