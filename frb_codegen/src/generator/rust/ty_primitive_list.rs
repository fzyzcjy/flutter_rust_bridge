use crate::generator::rust::ty::TypeRustGeneratorTrait;
use crate::generator::rust::ExternFuncCollector;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypePrimitiveListGenerator(pub IrTypePrimitiveList);

impl TypeRustGeneratorTrait for TypePrimitiveListGenerator {
    fn wire2api_body(&self) -> String {
        "unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }"
        .into()
    }

    fn wire_struct_fields(&self) -> Vec<String> {
        vec![
            format!("ptr: *mut {}", self.0.primitive.rust_wire_type()),
            "len: i32".to_string(),
        ]
    }

    fn allocate_funcs(&self, collector: &ExternFuncCollector) -> String {
        self.extern_func_collector.generate(
            &format!("new_{}", self.0.safe_ident()),
            &["len: i32"],
            Some(&format!(
                "{}{}",
                self.0.rust_wire_modifier(),
                self.0.rust_wire_type()
            )),
            &format!(
                "let ans = {} {{ ptr: support::new_leak_vec_ptr(Default::default(), len), len }};
                support::new_leak_box_ptr(ans)",
                self.0.rust_wire_type(),
            ),
        )
    }
}
