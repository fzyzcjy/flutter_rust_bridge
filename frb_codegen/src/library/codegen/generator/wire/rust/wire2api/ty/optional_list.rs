use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::wire2api::extern_func::CodeWithExternFunc;
use crate::codegen::generator::wire::rust::wire2api::misc::generate_class_from_fields;
use crate::codegen::generator::wire::rust::wire2api::ty::general_list::general_list_impl_wire2api_body;
use crate::codegen::generator::wire::rust::wire2api::ty::WireRustGeneratorWire2apiTrait;
use crate::codegen::ir::ty::IrTypeTrait;
use crate::library::codegen::generator::wire::rust::info::WireRustGeneratorInfoTrait;

impl<'a> WireRustGeneratorWire2apiTrait for OptionalListWireRustGenerator<'a> {
    fn generate_wire2api_class(&self) -> Option<String> {
        Some(generate_class_from_fields(
            self.ir.clone(),
            &self.context,
            &vec![
                format!(
                    "ptr: *mut *mut {}",
                    WireRustGenerator::new(*self.ir.inner.clone(), self.context.clone())
                        .rust_wire_type(Target::Io)
                ),
                "len: i32".to_string(),
            ],
        ))
    }

    fn generate_impl_wire2api_body(&self) -> Acc<Option<String>> {
        general_list_impl_wire2api_body()
    }

    fn generate_allocate_funcs(&self) -> Acc<Option<CodeWithExternFunc>> {
        Acc {
            io: Some(collector.generate(
                &format!("new_{}", self.ir.safe_ident()),
                [("len: i32", "int")],
                Some(&format!("*mut {}", self.ir.rust_wire_type(Target::Io))),
                &format!(
                    "let wrap = {} {{ ptr: support::new_leak_vec_ptr(core::ptr::null_mut(), len), len }};
                    support::new_leak_box_ptr(wrap)",
                    self.ir.rust_wire_type(Target::Io)
                ),
                Target::Io,
            )),
            ..Default::default()
        }
    }
}
