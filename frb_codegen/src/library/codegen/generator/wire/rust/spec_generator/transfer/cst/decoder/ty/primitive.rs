use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::IrTypeTrait;
use IrTypePrimitive::Unit;

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
            I64 | U64 => "::std::convert::TryInto::try_into(self.dyn_into::<flutter_rust_bridge::for_generated::js_sys::BigInt>().unwrap()).unwrap()".into(),
            _ => "self.unchecked_into_f64() as _".into(),
        })
    }

    fn rust_wire_type(&self, _target: Target) -> String {
        self.ir.rust_api_type()
    }
}
