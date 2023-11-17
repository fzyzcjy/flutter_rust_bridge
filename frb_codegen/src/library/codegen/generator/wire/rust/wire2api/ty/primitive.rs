use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::wire2api::ty::WireRustGeneratorWire2apiTrait;

impl<'a> WireRustGeneratorWire2apiTrait for PrimitiveWireRustGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> Acc<Option<String>> {
        "self".into()
    }

    fn generate_impl_wire2api_jsvalue_body(&self) -> Option<std::borrow::Cow<str>> {
        use IrTypePrimitive::*;
        Some(match self.ir {
            Bool => "self.is_truthy()".into(),
            I64 | U64 => "::std::convert::TryInto::try_into(self.dyn_into::<js_sys::BigInt>().unwrap()).unwrap()".into(),
            _ => "self.unchecked_into_f64() as _".into(),
        })
    }
}
