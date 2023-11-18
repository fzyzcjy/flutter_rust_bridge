use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::extern_func::{ExternFunc, ExternFuncParam};
use crate::codegen::generator::wire::rust::wire2api::misc::generate_class_from_fields;
use crate::codegen::generator::wire::rust::wire2api::ty::general_list::general_list_impl_wire2api_body;
use crate::codegen::generator::wire::rust::wire2api::ty::WireRustGeneratorWire2apiTrait;
use crate::codegen::generator::wire::rust::wire_rust_code::WireRustCode;
use crate::codegen::ir::ty::IrTypeTrait;
use crate::library::codegen::generator::wire::rust::misc::ty::WireRustGeneratorMiscTrait;

impl<'a> WireRustGeneratorWire2apiTrait for OptionalListWireRustGenerator<'a> {
    fn generate_wire2api_class(&self) -> Option<String> {
        Some(generate_class_from_fields(
            self.ir.clone(),
            self.context,
            &vec![
                format!(
                    "ptr: *mut *mut {}",
                    WireRustGenerator::new(self.ir.inner.clone(), self.context)
                        .rust_wire_type(Target::Io)
                ),
                "len: i32".to_string(),
            ],
        ))
    }

    fn generate_impl_wire2api_body(&self) -> Acc<Option<String>> {
        general_list_impl_wire2api_body()
    }

    fn generate_allocate_funcs(&self) -> Acc<WireRustCode> {
        Acc {
            io: ExternFunc {
                func_name: format!("new_{}", self.ir.safe_ident()),
                params: vec![ExternFuncParam {
                    name: "len".to_owned(),
                    rust_type: "i32".to_owned(),
                    dart_type: Some("int".to_owned()),
                }],
                return_type:  Some(format!("*mut {}", self.rust_wire_type(Target::Io))),
                body: format!(
                    "let wrap = {} {{ ptr: support::new_leak_vec_ptr(core::ptr::null_mut(), len), len }};
                    support::new_leak_box_ptr(wrap)",
                    self.rust_wire_type(Target::Io)
                ),
                target: Target::Io,
            }.into(),
            ..Default::default()
        }
    }
}
