use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::decoder::impl_new_with_nullptr::generate_impl_new_with_nullptr_code_block;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::decoder::misc::{
    generate_class_from_fields, rust_wire_type_add_prefix_or_js_value, JS_VALUE,
};
use crate::codegen::generator::wire::rust::spec_generator::extern_func::{
    ExternFunc, ExternFuncParam,
};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::decoder::ty::WireRustTransferCstGeneratorDecoderTrait;
use crate::codegen::ir::ty::{IrType, IrTypeTrait};
use itertools::Itertools;
use std::borrow::Cow;

impl<'a> WireRustTransferCstGeneratorDecoderTrait for RustOpaqueWireRustTransferCstGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> Acc<Option<String>> {
        Acc {
            io: Some(generalized_rust_opaque_generate_impl_wire2api_body().into()),
            ..Default::default()
        }
    }

    fn generate_impl_wire2api_jsvalue_body(&self) -> Option<Cow<str>> {
        Some(generalized_rust_opaque_generate_impl_wire2api_body().into())
    }

    fn rust_wire_type(&self, target: Target) -> String {
        dart_opaque_or_generalized_rust_opaque_rust_wire_type(target)
    }
}

pub(super) fn generalized_rust_opaque_generate_impl_wire2api_body() -> &'static str {
    r#"unsafe { flutter_rust_bridge::for_generated::wire2api_rust_opaque(self) }"#
}

pub(super) fn dart_opaque_or_generalized_rust_opaque_rust_wire_type(target: Target) -> String {
    match target {
        Target::Io => "*const std::ffi::c_void",
        Target::Wasm => JS_VALUE,
    }
    .into()
}
