use crate::generator::rust::ty::TypeRustGeneratorTrait;
use crate::generator::rust::ExternFuncCollector;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypeGeneralListGenerator(IrTypeGeneralList);

impl TypeGeneralListGenerator {
    pub const WIRE2API_BODY: &'static str = "
            let vec = unsafe {
                let wrap = support::box_from_leak_ptr(self);
                support::vec_from_leak_ptr(wrap.ptr, wrap.len)
            };
            vec.into_iter().map(Wire2Api::wire2api).collect()";
}

impl TypeRustGeneratorTrait for TypeGeneralListGenerator {
    fn wire2api_body(&self) -> String {
        TypeGeneralListGenerator::WIRE2API_BODY.to_string()
    }

    fn wire_struct_fields(&self) -> Vec<String> {
        vec![
            format!(
                "ptr: *mut {}{}",
                self.0.inner.rust_ptr_modifier(),
                self.0.inner.rust_wire_type()
            ),
            "len: i32".to_string(),
        ]
    }

    fn allocate_funcs(&self, collector: &ExternFuncCollector) -> String {
        self.generate_list_allocate_func(&ty.safe_ident(), list, &list.inner)
    }

    fn imports(&self) -> Option<String> {
        self.generate_import(&self.0.inner, api_file)
    }
}
