use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::misc::JS_VALUE;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::IrTypeTrait;
use IrTypePrimitive::Unit;
use crate::codegen::ir::ty::primitive::IrTypePrimitive::{I64, Isize};

impl<'a> WireRustCodecCstGeneratorDecoderTrait for PrimitiveWireRustCodecCstGenerator<'a> {
    fn generate_impl_decode_body(&self) -> Acc<Option<String>> {
        match self.ir {
            Unit => Acc::new(|_| None),
            _ => "self".into(),
        }
    }

    fn generate_impl_decode_jsvalue_body(&self) -> Option<std::borrow::Cow<str>> {
        use IrTypePrimitive::*;
        Some(match &self.ir {
            Unit => return None,
            Bool => "self.is_truthy()".into(),
            I64 | Isize => "::std::convert::TryInto::try_into::<i64>(self).unwrap() as _".into(),
            U64 | Usize => "::std::convert::TryInto::try_into::<u64>(self).unwrap() as _".into(),
            _ => "self.unchecked_into_f64() as _".into(),
        })
    }

    fn rust_wire_type(&self, target: Target) -> String {
        use IrTypePrimitive::*;
        if target == Target::Web && matches!(self.ir, I64 | U64 | Isize | Usize) {
            return JS_VALUE.into();
        }
        self.ir.rust_api_type()
    }
}
