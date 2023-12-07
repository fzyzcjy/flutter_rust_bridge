use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::is_js_value;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::decoder::ty::WireRustTransferCstGeneratorDecoderTrait;

impl<'a> WireRustTransferCstGeneratorDecoderTrait for OptionalWireRustTransferCstGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> Acc<Option<String>> {
        Acc {
            wasm: (!is_js_value(&self.ir.inner)
                && !self.ir.is_primitive()
                && !self.ir.is_boxed_primitive())
            .then(|| "self.map(Wire2Api::wire2api)".into()),
            ..Default::default()
        }
    }

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
}
