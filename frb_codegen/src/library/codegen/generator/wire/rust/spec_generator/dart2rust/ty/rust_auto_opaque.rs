use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::dart2rust::impl_new_with_nullptr::generate_impl_new_with_nullptr_code_block;
use crate::codegen::generator::wire::rust::spec_generator::dart2rust::misc::{
    generate_class_from_fields, rust_wire_type_add_prefix_or_js_value, JS_VALUE,
};
use crate::codegen::generator::wire::rust::spec_generator::dart2rust::ty::rust_opaque::{
    generalized_rust_opaque_generate_impl_wire2api_body, generalized_rust_opaque_rust_wire_type,
    generate_rust_arc_functions,
};
use crate::codegen::generator::wire::rust::spec_generator::dart2rust::ty::WireRustGeneratorDart2RustTrait;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::{
    ExternFunc, ExternFuncParam,
};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;
use std::borrow::Cow;

impl<'a> WireRustGeneratorDart2RustTrait for RustAutoOpaqueWireRustGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> Acc<Option<String>> {
        Acc {
            io: Some(generalized_rust_opaque_generate_impl_wire2api_body().into()),
            ..Default::default()
        }
    }

    fn generate_impl_wire2api_jsvalue_body(&self) -> Option<Cow<str>> {
        Some(generalized_rust_opaque_generate_impl_wire2api_body().into())
    }

    fn generate_related_funcs(&self) -> Acc<WireRustOutputCode> {
        generate_rust_arc_functions(self.ir.clone().into(), &*self.ir.inner)
    }

    fn rust_wire_type(&self, target: Target) -> String {
        generalized_rust_opaque_rust_wire_type(target)
    }
}
