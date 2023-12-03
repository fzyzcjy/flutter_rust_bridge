use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::dart2rust::impl_new_with_nullptr::generate_impl_new_with_nullptr_code_block;
use crate::codegen::generator::wire::rust::spec_generator::dart2rust::misc::{
    generate_class_from_fields, rust_wire_type_add_prefix_or_js_value, JS_VALUE,
};
use crate::codegen::generator::wire::rust::spec_generator::dart2rust::ty::WireRustGeneratorDart2RustTrait;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::{
    ExternFunc, ExternFuncParam,
};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::ty::{IrType, IrTypeTrait};
use itertools::Itertools;
use std::borrow::Cow;

impl<'a> WireRustGeneratorDart2RustTrait for RustOpaqueWireRustGenerator<'a> {
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
        generate_rust_arc_functions(self.ir.clone().into())
    }

    fn rust_wire_type(&self, target: Target) -> String {
        generalized_rust_opaque_rust_wire_type(target)
    }
}

pub(super) fn generalized_rust_opaque_generate_impl_wire2api_body() -> &'static str {
    r#"unsafe { flutter_rust_bridge::for_generated::wire2api_rust_opaque(self) }"#
}

pub(super) fn generalized_rust_opaque_rust_wire_type(target: Target) -> String {
    match target {
        Target::Io => "*const std::ffi::c_void",
        Target::Wasm => JS_VALUE,
    }
    .into()
}

pub(super) fn generate_rust_arc_functions(ir: IrType) -> Acc<WireRustOutputCode> {
    let generate_impl = |target| -> WireRustOutputCode {
        ["increment", "decrement"].into_iter()
            .map(|op|
                     ExternFunc {
                         func_name: format!("rust_arc_{op}_strong_count_{}", ir.safe_ident()),
                         params: vec![ExternFuncParam {
                             name: "ptr".to_owned(),
                             rust_type: "*const std::ffi::c_void".to_owned(),
                             dart_type: "dynamic".into(),
                         }.clone()],
                         return_type: None,
                         body: format!(
                             "unsafe {{ flutter_rust_bridge::for_generated::rust_arc_{op}_strong_count::<{}>(ptr); }}",
                             ir.inner.rust_api_type()
                         ),
                         target,
                     },
            )
            .collect_vec()
            .into()
    };

    Acc {
        io: generate_impl(Target::Io),
        wasm: generate_impl(Target::Wasm),
        ..Default::default()
    }
}
