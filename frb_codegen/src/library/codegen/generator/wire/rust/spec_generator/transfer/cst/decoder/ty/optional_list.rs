use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::decoder::misc::{
    generate_class_from_fields, rust_wire_type_add_prefix_or_js_value,
};
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::decoder::ty::general_list::general_list_impl_decode_body;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::{
    ExternFunc, ExternFuncParam,
};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::decoder::ty::WireRustTransferCstGeneratorDecoderTrait;
use crate::codegen::ir::ty::IrTypeTrait;

impl<'a> WireRustTransferCstGeneratorDecoderTrait for OptionalListWireRustTransferCstGenerator<'a> {
    fn generate_allocate_funcs(&self) -> Acc<WireRustOutputCode> {
        Acc {
            io: ExternFunc {
                func_name: format!("cst_new_{}", self.ir.safe_ident()),
                params: vec![ExternFuncParam {
                    name: "len".to_owned(),
                    rust_type: "i32".to_owned(),
                    dart_type: "int".to_owned(),
                }],
                return_type:  Some(format!("*mut {}", self.rust_wire_type(Target::Io))),
                body: format!(
                    "let wrap = {} {{ ptr: flutter_rust_bridge::for_generated::new_leak_vec_ptr(core::ptr::null_mut(), len), len }};
                    flutter_rust_bridge::for_generated::new_leak_box_ptr(wrap)",
                    self.rust_wire_type(Target::Io)
                ),
                target: Target::Io,
            }.into(),
            ..Default::default()
        }
    }
}
