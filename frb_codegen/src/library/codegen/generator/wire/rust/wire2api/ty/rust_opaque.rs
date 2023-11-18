use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::wire2api::misc::generate_class_from_fields;
use crate::codegen::generator::wire::rust::wire2api::ty::WireRustGeneratorWire2apiTrait;
use std::borrow::Cow;

impl<'a> WireRustGeneratorWire2apiTrait for RustOpaqueWireRustGenerator<'a> {
    fn generate_wire2api_class(&self) -> Option<String> {
        Some(generate_class_from_fields(
            self.ir.clone(),
            &self.context,
            &vec!["ptr: *const core::ffi::c_void".to_owned()],
        ))
    }

    fn generate_impl_wire2api_body(&self) -> Acc<Option<String>> {
        Acc {
            io: Some(
                "unsafe {
                support::opaque_from_dart(self.ptr as _)
            }"
                .into(),
            ),
            ..Default::default()
        }
    }

    fn generate_impl_wire2api_jsvalue_body(&self) -> Option<Cow<str>> {
        Some(
            r#"
            #[cfg(target_pointer_width = "64")]
            {
                compile_error!("64-bit pointers are not supported.");
            }
    
            unsafe {
                support::opaque_from_dart((self.as_f64().unwrap() as usize) as _)
            }"#
            .into(),
        )
    }

    fn generate_impl_new_with_nullptr(&self) -> Option<CodeWithExternFunc> {
        format!(
            "impl NewWithNullPtr for {} {{
                fn new_with_null_ptr() -> Self {{
                    Self {{ ptr: core::ptr::null() }}
                }}
            }}",
            self.ir.rust_wire_type(crate::target::Target::Io)
        )
    }
}
