use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::misc::JS_VALUE;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::ir::mir::ty::MirTypeTrait;
use MirTypePrimitive::Unit;

impl WireRustCodecCstGeneratorDecoderTrait for PrimitiveWireRustCodecCstGenerator<'_> {
    fn generate_impl_decode_body(&self) -> Acc<Option<String>> {
        match self.mir {
            Unit => Acc::new(|_| None),
            _ => "self".into(),
        }
    }

    fn generate_impl_decode_jsvalue_body(&self) -> Option<std::borrow::Cow<'_, str>> {
        use MirTypePrimitive::*;
        Some(match &self.mir {
            Unit => return None,
            Bool => "self.is_truthy()".into(),
            I64 | Isize => "::std::convert::TryInto::<i64>::try_into(self).unwrap() as _".into(),
            U64 | Usize => "::std::convert::TryInto::<u64>::try_into(self).unwrap() as _".into(),
            _ => "self.unchecked_into_f64() as _".into(),
        })
    }

    fn rust_wire_type(&self, target: Target) -> String {
        use MirTypePrimitive::*;
        if target == Target::Web && matches!(self.mir, I64 | U64 | Isize | Usize) {
            return JS_VALUE.into();
        }
        self.mir.rust_api_type()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::misc::target::Target;
    use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::test_utils;

    /// Emits each primitive's CST decoder and web wire-type specialization.
    #[test]
    fn primitive_decoder_covers_unit_boolean_large_integer_and_float() {
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

        let unit = PrimitiveWireRustCodecCstGenerator::new(MirTypePrimitive::Unit, context);
        assert_eq!(unit.generate_impl_decode_body().io, None);
        assert_eq!(unit.generate_impl_decode_jsvalue_body(), None);

        let boolean = PrimitiveWireRustCodecCstGenerator::new(MirTypePrimitive::Bool, context);
        assert_eq!(
            boolean.generate_impl_decode_body().common.as_deref(),
            Some("self")
        );
        assert_eq!(
            boolean.generate_impl_decode_jsvalue_body().as_deref(),
            Some("self.is_truthy()")
        );

        let signed = PrimitiveWireRustCodecCstGenerator::new(MirTypePrimitive::I64, context);
        assert_eq!(
            signed.generate_impl_decode_jsvalue_body().as_deref(),
            Some("::std::convert::TryInto::<i64>::try_into(self).unwrap() as _")
        );
        assert_eq!(signed.rust_wire_type(Target::Web), JS_VALUE);

        let float = PrimitiveWireRustCodecCstGenerator::new(MirTypePrimitive::F64, context);
        assert_eq!(
            float.generate_impl_decode_jsvalue_body().as_deref(),
            Some("self.unchecked_into_f64() as _")
        );
        assert_eq!(float.rust_wire_type(Target::Web), "f64");
    }
}
