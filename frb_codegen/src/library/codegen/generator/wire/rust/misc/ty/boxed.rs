use crate::codegen::generator::misc::{is_js_value, Target};
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::misc::misc::JS_VALUE;
use crate::codegen::generator::wire::rust::misc::ty::WireRustGeneratorMiscTrait;

impl<'a> WireRustGeneratorMiscTrait for BoxedWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        if target == Target::Wasm && self.ir.inner.is_primitive() {
            JS_VALUE.into()
        } else {
            WireRustGenerator::new(self.ir.inner.clone(), self.context).rust_wire_type(target)
        }
    }

    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        (target != Target::Wasm)
            || !is_js_value(&self.ir.inner)
                && !self.ir.inner.is_array()
                && !self.ir.inner.is_primitive()
    }

    // TODO https://github.com/fzyzcjy/yplusplus/issues/11145#issuecomment-1816273032
    // fn wrapper_struct_name(&self) -> Option<String> {
    //     let src = TypeRustGenerator::new(
    //         *self.ir.inner.clone(),
    //         self.context.ir_pack,
    //         self.context.config,
    //     );
    //     src.wrapper_struct_name()
    // }

    // TODO rm this, since we will visit all sub-types to generate
    // fn generate_imports(&self) -> Option<Vec<String>> {
    //     generate_import(&self.ir.inner, self.context.ir_pack, self.context.config)
    // }
}
