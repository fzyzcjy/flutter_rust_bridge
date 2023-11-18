use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::extern_func::{ExternFunc, ExternFuncParam};
use crate::codegen::generator::wire::rust::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::wire2api::impl_new_with_nullptr::generate_impl_new_with_nullptr_code_block;
use crate::codegen::generator::wire::rust::wire2api::misc::generate_class_from_fields;
use crate::codegen::generator::wire::rust::wire2api::ty::WireRustGeneratorWire2apiTrait;
use crate::codegen::ir::ty::IrTypeTrait;
use crate::library::codegen::generator::wire::rust::misc::ty::WireRustGeneratorMiscTrait;
use std::borrow::Cow;

impl<'a> WireRustGeneratorWire2apiTrait for RustOpaqueWireRustGenerator<'a> {
    fn generate_wire2api_class(&self) -> Option<String> {
        Some(generate_class_from_fields(
            self.ir.clone(),
            self.context,
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

    fn generate_impl_new_with_nullptr(&self) -> Option<WireRustOutputCode> {
        Some(
            generate_impl_new_with_nullptr_code_block(
                self.ir.clone(),
                self.context,
                "Self { ptr: core::ptr::null() }",
                false,
            )
            .into(),
        )
    }

    fn generate_allocate_funcs(&self) -> Acc<WireRustOutputCode> {
        let rust_wire = self.rust_wire_type(Target::Io);

        Acc {
            io: ExternFunc {
                func_name: format!("new_{}", self.ir.safe_ident()),
                params: vec![],
                return_type: Some(format!(
                    "{}{rust_wire}",
                    self.rust_wire_modifier(Target::Io),
                )),
                body: format!("{rust_wire}::new_with_null_ptr()"),
                target: Target::Io,
            }
            .into(),
            ..Default::default()
        }
    }

    fn generate_related_funcs(&self) -> Acc<WireRustOutputCode> {
        let generate_impl = |target| -> WireRustOutputCode {
            vec![
                ExternFunc {
                    func_name: format!("drop_opaque_{}", self.ir.safe_ident()),
                    params: vec![ExternFuncParam {
                        name: "ptr".to_owned(),
                        rust_type: "*const c_void".to_owned(),
                        dart_type: None,
                    }],
                    return_type: None,
                    body: format!(
                        "unsafe {{Arc::<{}>::decrement_strong_count(ptr as _);}}",
                        self.ir.inner.rust_api_type()
                    ),
                    target,
                },
                ExternFunc {
                    func_name: format!("share_opaque_{}", self.ir.safe_ident()),
                    params: vec![ExternFuncParam {
                        name: "ptr".to_owned(),
                        rust_type: "*const c_void".to_owned(),
                        dart_type: None,
                    }],
                    return_type: Some("*const c_void".to_string()),
                    body: format!(
                        "unsafe {{Arc::<{}>::increment_strong_count(ptr as _); ptr}}",
                        self.ir.inner.rust_api_type()
                    ),
                    target,
                },
            ]
            .into()
        };
        Acc {
            io: generate_impl(Target::Io),
            wasm: generate_impl(Target::Wasm),
            ..Default::default()
        }
    }
}
