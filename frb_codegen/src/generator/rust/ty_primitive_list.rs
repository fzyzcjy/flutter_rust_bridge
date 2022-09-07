use crate::generator::rust::ty::*;
use crate::generator::rust::ExternFuncCollector;
use crate::ir::*;
use crate::target::Acc;
use crate::target::Target;
use crate::type_rust_generator_struct;
use crate::utils::BlockIndex;

type_rust_generator_struct!(TypePrimitiveListGenerator, IrTypePrimitiveList);

impl TypeRustGeneratorTrait for TypePrimitiveListGenerator<'_> {
    fn wire2api_body(&self) -> Acc<Option<String>> {
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

    fn wire2api_jsvalue(&self) -> Option<std::borrow::Cow<str>> {
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

    fn wire_struct_fields(&self) -> Option<Vec<String>> {
        Some(vec![
            format!("ptr: *mut {}", self.ir.primitive.rust_wire_type(Target::Io)),
            "len: i32".to_string(),
        ])
    }

    fn allocate_funcs(
        &self,
        collector: &mut ExternFuncCollector,
        block_index: BlockIndex,
    ) -> Acc<Option<String>> {
        Acc {
            io: Some(collector.generate(
                &format!("new_{}_{}", self.ir.safe_ident(), block_index),
                [("len: i32", "int")],
                Some(&format!(
                    "{}{}",
                    self.ir.rust_wire_modifier(Target::Io),
                    self.ir.rust_wire_type(Target::Io)
                )),
                &format!(
                    "let ans = {} {{ ptr: support::new_leak_vec_ptr(Default::default(), len), len }};
                    support::new_leak_box_ptr(ans)",
                    self.ir.rust_wire_type(Target::Io),
                ),
                Target::Io,
            )),
            ..Default::default()
        }
    }
}
