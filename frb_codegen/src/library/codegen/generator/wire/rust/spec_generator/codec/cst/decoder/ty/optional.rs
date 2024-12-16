use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::is_js_value;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;

impl<'a> WireRustCodecCstGeneratorDecoderTrait for OptionalWireRustCodecCstGenerator<'a> {
    fn generate_impl_decode_body(&self) -> Acc<Option<String>> {
        Acc {
            web: (!is_js_value(&self.mir.inner)
                && !self.mir.is_primitive()
                && !self.mir.is_boxed_primitive())
            .then(|| "self.map(CstDecode::cst_decode)".into()),
            ..Default::default()
        }
    }

    fn rust_wire_type(&self, target: Target) -> String {
        let inner_generator = WireRustCodecCstGenerator::new(self.mir.inner.clone(), self.context);

        if inner_generator.rust_wire_is_pointer(target)
            || (target == Target::Web)
                && (is_js_value(&self.mir.inner)
                    || self.mir.is_primitive()
                    || self.mir.is_boxed_primitive())
        {
            inner_generator.rust_wire_type(target)
        } else {
            format!("Option<{}>", inner_generator.rust_wire_type(target))
        }
    }

    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        target != Target::Web
            || WireRustCodecCstGenerator::new(self.mir.inner.clone(), self.context)
                .rust_wire_is_pointer(target)
    }
}
