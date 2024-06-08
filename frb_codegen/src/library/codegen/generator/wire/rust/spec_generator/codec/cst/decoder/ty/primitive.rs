use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::misc::JS_VALUE;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::ir::mir::ty::MirTypeTrait;
use MirTypePrimitive::Unit;

impl<'a> WireRustCodecCstGeneratorDecoderTrait for PrimitiveWireRustCodecCstGenerator<'a> {
    fn generate_impl_decode_body(&self) -> Acc<Option<String>> {
        match self.mir {
            Unit => Acc::new(|_| None),
            _ => "self".into(),
        }
    }

    fn generate_impl_decode_jsvalue_body(&self) -> Option<std::borrow::Cow<str>> {
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
