use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::wire2api::ty::WireRustGeneratorWire2apiTrait;

impl<'a> WireRustGeneratorWire2apiTrait for BoxedWireRustGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> Acc<Option<String>> {
        let box_inner = self.ir.inner.as_ref();
        let exist_in_real_api = self.ir.exist_in_real_api;
        Acc::new(|target| match (target, self.ir.inner.as_ref()) {
            (Io, IrType::Primitive(_)) => Some(
                if exist_in_real_api {
                    "unsafe { support::box_from_leak_ptr(self) }"
                } else {
                    "unsafe { *support::box_from_leak_ptr(self) }"
                }
                .into(),
            ),
            (Io | Wasm, ir) if ir.is_array() => Some(format!(
                "Wire2Api::<{}>::wire2api(self).into()",
                box_inner.rust_api_type()
            )),
            (Io, _) => Some(format!(
                "let wrap = unsafe {{ support::box_from_leak_ptr(self) }};
                Wire2Api::<{}>::wire2api(*wrap).into()",
                box_inner.rust_api_type()
            )),
            _ => None,
        })
    }

    fn generate_impl_wire2api_jsvalue_body(&self) -> Option<std::borrow::Cow<str>> {
        (self.ir.exist_in_real_api).then(|| match &*self.ir.inner {
            IrType::Delegate(IrTypeDelegate::PrimitiveEnum { repr, .. }) => format!(
                "let ptr: Box<{}> = self.wire2api(); Box::new(ptr.wire2api())",
                repr.rust_api_type()
            )
            .into(),
            IrType::Delegate(IrTypeDelegate::Array(array)) => format!(
                "let vec: Vec<{}> = self.wire2api(); Box::new(support::from_vec_to_array(vec))",
                array.inner().rust_api_type()
            )
            .into(),
            _ => "Box::new(self.wire2api())".into(),
        })
    }
}
