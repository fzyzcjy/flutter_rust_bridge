use crate::codegen::generator::misc::{is_js_value, Target};
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;

impl<'a> WireRustGeneratorMiscTrait for OptionalWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        let inner_generator = WireRustGenerator::new(self.ir.inner.clone(), self.context);

        if inner_generator.rust_wire_is_pointer(target)
            || (target == Target::Wasm)
                && (is_js_value(&self.ir.inner)
                    || self.ir.is_primitive()
                    || self.ir.is_boxed_primitive())
        {
            inner_generator.rust_wire_type(target)
        } else {
            format!("Option<{}>", inner_generator.rust_wire_type(target))
        }
    }

    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        target != Target::Wasm
            || WireRustGenerator::new(self.ir.inner.clone(), self.context)
                .rust_wire_is_pointer(target)
    }

    // TODO rm this, since we will visit all sub-types to generate
    // fn generate_imports(&self) -> Option<Vec<String>> {
    //     generate_import(&self.ir.inner, self.context.ir_pack, self.context.config)
    // }
}
