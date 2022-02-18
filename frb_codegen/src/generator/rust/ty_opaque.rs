use crate::generator::rust::ty::*;
use crate::ir::*;
use crate::type_rust_generator_struct;

use super::ExternFuncCollector;

type_rust_generator_struct!(TypeOpaqueGenerator, IrTypeOpaque);

impl TypeRustGeneratorTrait for TypeOpaqueGenerator<'_> {
    fn wire2api_body(&self) -> Option<String> {
        Some("unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }".into())
    }

    fn wire_struct_fields(&self) -> Option<Vec<String>> {
        Some(vec!["ptr: *const core::ffi::c_void".to_owned()])
    }

    fn allocate_funcs(&self, collector: &mut ExternFuncCollector) -> String {
        let rust_wire = self.ir.rust_wire_type();
        collector.generate(
            &format!("new_{}", self.ir.safe_ident()),
            &[],
            Some(&format!("{}{}", self.ir.rust_wire_modifier(), rust_wire)),
            &format!(
                "support::new_leak_box_ptr({}::new_with_null_ptr())",
                rust_wire,
            ),
        )
    }

    fn impl_intodart(&self) -> String {
        "".to_string()
    }

    fn new_with_nullptr(&self, _: &mut ExternFuncCollector) -> String {
        format!(
            "impl NewWithNullPtr for {} {{
                fn new_with_null_ptr() -> Self {{
                    Self {{ ptr: core::ptr::null() }}
                }}
            }}",
            self.ir.rust_wire_type()
        )
    }
}
