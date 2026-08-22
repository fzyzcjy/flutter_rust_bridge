use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::is_js_value;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;

impl WireRustCodecCstGeneratorDecoderTrait for OptionalWireRustCodecCstGenerator<'_> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::test_utils;
    use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::misc::JS_VALUE;
    use crate::codegen::ir::mir::ty::optional::MirTypeOptional;
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use crate::codegen::ir::mir::ty::primitive_list::MirTypePrimitiveList;
    use crate::codegen::ir::mir::ty::MirType;

    /// Preserves optional scalar wire types and maps non-JavaScript web objects.
    #[test]
    fn optional_decoder_covers_scalar_and_object_web_branches() {
        let pack = test_utils::pack();
        let wire_dart_config = test_utils::wire_dart_config(true);
        let wire_rust_config = test_utils::wire_rust_config(true);
        let api_dart_config = test_utils::api_dart_config();
        let dart_context = test_utils::context(
            &pack,
            &wire_dart_config,
            &wire_rust_config,
            &api_dart_config,
        );
        let context = dart_context.as_wire_rust_context();

        let scalar = OptionalWireRustCodecCstGenerator::new(
            MirTypeOptional::new_with_boxed_wrapper(MirType::Primitive(MirTypePrimitive::I32)),
            context,
        );
        assert_eq!(scalar.generate_impl_decode_body().web, None);
        assert_eq!(scalar.rust_wire_type(Target::Web), JS_VALUE);

        let object = OptionalWireRustCodecCstGenerator::new(
            MirTypeOptional {
                inner: Box::new(MirType::PrimitiveList(MirTypePrimitiveList {
                    primitive: MirTypePrimitive::I32,
                    strict_dart_type: true,
                })),
            },
            context,
        );
        assert_eq!(
            object.generate_impl_decode_body().web.as_deref(),
            Some("self.map(CstDecode::cst_decode)")
        );
        assert_eq!(object.rust_wire_type(Target::Web), "Option<Box<[i32]>>");
    }
}
