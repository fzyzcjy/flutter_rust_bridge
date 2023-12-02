use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::dart2rust::ty::WireRustGeneratorDart2RustTrait;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::IrTypeTrait;

impl<'a> WireRustGeneratorDart2RustTrait for PrimitiveWireRustGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> Acc<Option<String>> {
        "self".into()
    }

    fn generate_impl_wire2api_jsvalue_body(&self) -> Option<std::borrow::Cow<str>> {
        use IrTypePrimitive::*;
        Some(match &self.ir {
            Bool => "self.is_truthy()".into(),
            I64 | U64 => "::std::convert::TryInto::try_into(self.dyn_into::<flutter_rust_bridge::for_generated::js_sys::BigInt>().unwrap()).unwrap()".into(),
            _ => "self.unchecked_into_f64() as _".into(),
        })
    }

    fn rust_wire_type(&self, _target: Target) -> String {
        self.ir.rust_api_type()
    }
}
