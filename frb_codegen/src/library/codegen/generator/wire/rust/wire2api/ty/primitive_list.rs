use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::wire2api::misc::generate_class_from_fields;
use crate::codegen::generator::wire::rust::wire2api::ty::WireRustGeneratorWire2apiTrait;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::library::codegen::generator::wire::rust::info::WireRustGeneratorInfoTrait;

impl<'a> WireRustGeneratorWire2apiTrait for PrimitiveListWireRustGenerator<'a> {
    fn generate_wire2api_class(&self) -> Option<String> {
        Some(generate_class_from_fields(
            self.ir.clone(),
            &self.context,
            &vec![
                format!(
                    "ptr: *mut {}",
                    WireRustGenerator::new(self.ir.primitive.clone().into(), self.context.clone())
                        .rust_wire_type(Target::Io)
                ),
                "len: i32".to_string(),
            ],
        ))
    }

    fn generate_impl_wire2api_body(&self) -> Acc<Option<String>> {
        Acc {
            wasm: Some("self.into_vec()".into()),
            io: Some(
                "unsafe {
                    let wrap = support::box_from_leak_ptr(self);
                    support::vec_from_leak_ptr(wrap.ptr, wrap.len)
                }"
                .into(),
            ),
            ..Default::default()
        }
    }

    fn generate_impl_wire2api_jsvalue_body(&self) -> Option<std::borrow::Cow<str>> {
        match self.ir.primitive {
            IrTypePrimitive::Bool | IrTypePrimitive::Unit => Some("todo!()".into()),
            IrTypePrimitive::I64 | IrTypePrimitive::U64 => Some(
                format!(
                    "let buf = self.dyn_into::<{}>().unwrap();
                    let buf = js_sys::Uint8Array::new(&buf.buffer());
                    support::slice_from_byte_buffer(buf.to_vec()).into()",
                    self.ir.rust_wasm_wire_type()
                )
                .into(),
            ),
            _ => Some(
                format!(
                    "self.unchecked_into::<{}>().to_vec().into()",
                    self.ir.rust_wasm_wire_type()
                )
                .into(),
            ),
        }
    }
}
