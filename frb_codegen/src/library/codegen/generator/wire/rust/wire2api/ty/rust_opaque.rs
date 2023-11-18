use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::wire2api::extern_func::CodeWithExternFunc;
use crate::codegen::generator::wire::rust::wire2api::impl_new_with_nullptr::generate_impl_new_with_nullptr_code_block;
use crate::codegen::generator::wire::rust::wire2api::misc::generate_class_from_fields;
use crate::codegen::generator::wire::rust::wire2api::ty::WireRustGeneratorWire2apiTrait;
use crate::library::codegen::generator::wire::rust::info::WireRustGeneratorInfoTrait;
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
        Some(CodeWithExternFunc::code(
            generate_impl_new_with_nullptr_code_block(
                self.ir.clone(),
                &self.context,
                "Self { ptr: core::ptr::null() }",
                false,
            ),
        ))
    }
}
