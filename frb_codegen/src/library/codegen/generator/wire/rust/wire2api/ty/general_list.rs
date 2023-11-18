use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::wire2api::extern_func::{
    CodeWithExternFunc, ExternFunc, ExternFuncParam,
};
use crate::codegen::generator::wire::rust::wire2api::misc::generate_class_from_fields;
use crate::codegen::generator::wire::rust::wire2api::ty::WireRustGeneratorWire2apiTrait;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::general_list::IrTypeGeneralList;
use crate::codegen::ir::ty::IrType::{Delegate, Optional};
use crate::codegen::ir::ty::{IrType, IrTypeTrait};
use crate::library::codegen::generator::wire::rust::info::WireRustGeneratorInfoTrait;

impl<'a> WireRustGeneratorWire2apiTrait for GeneralListWireRustGenerator<'a> {
    fn generate_wire2api_class(&self) -> Option<String> {
        Some(generate_class_from_fields(
            self.ir.clone(),
            &self.context,
            &vec![
                format!(
                    "ptr: *mut {}{}",
                    general_list_maybe_extra_pointer_indirection(&self.ir),
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
            io: Some(
                generate_list_generate_allocate_func(
                    &self.ir.safe_ident(),
                    &self.ir.into(),
                    &self.ir.inner,
                    &self.context,
                )
                .into(),
            ),
            ..Default::default()
        }
    }
}

/// Does it need additional indirection for types put behind a vector
pub(crate) fn general_list_maybe_extra_pointer_indirection(ir: &IrTypeGeneralList) -> &'static str {
    if matches!(*ir.inner, Optional(_) | Delegate(IrTypeDelegate::String)) {
        "*mut "
    } else {
        ""
    }
}

pub(crate) fn general_list_impl_wire2api_body() -> Acc<Option<String>> {
    Acc {
        wasm: Some(WIRE2API_BODY_WASM.to_owned()),
        io: Some(WIRE2API_BODY_IO.to_owned()),
        ..Default::default()
    }
}

const WIRE2API_BODY_IO: &'static str = "
    let vec = unsafe {
        let wrap = support::box_from_leak_ptr(self);
        support::vec_from_leak_ptr(wrap.ptr, wrap.len)
    };
    vec.into_iter().map(Wire2Api::wire2api).collect()
";
const WIRE2API_BODY_WASM: &'static str =
    "self.dyn_into::<JsArray>().unwrap().iter().map(Wire2Api::wire2api).collect()";

pub(crate) fn generate_list_generate_allocate_func(
    safe_ident: &str,
    list: &IrType,
    inner: &IrType,
    context: &WireRustGeneratorContext,
) -> ExternFunc {
    let list_generator = WireRustGenerator::new(list.clone(), context.clone());

    // let wasm = false;
    ExternFunc {
        func_name: format!("new_{safe_ident}"),
        params: vec![ExternFuncParam {
            name: "len".to_owned(),
            rust_type: "i32".to_owned(),
            dart_type: Some("int".to_owned()),
        }],
        return_type: Some(
            [
                list_generator.rust_wire_modifier(Target::Io).as_str(),
                list_generator.rust_wire_type(Target::Io).as_str(),
            ]
            .concat(),
        ),
        body: format!(
            "let wrap = {} {{ ptr: support::new_leak_vec_ptr({}, len), len }};
                support::new_leak_box_ptr(wrap)",
            list_generator.rust_wire_type(Target::Io),
            if inner.is_primitive() {
                // A primitive enum list can use a default value since
                // `<i32>::new_with_null_ptr()` isn't implemented.
                "Default::default()".to_string()
            } else {
                format!(
                    "<{}{}>::new_with_null_ptr()",
                    general_list_maybe_extra_pointer_indirection(&IrTypeGeneralList {
                        inner: Box::new(inner.clone())
                    }),
                    WireRustGenerator::new(inner.clone(), context.clone())
                        .rust_wire_type(Target::Io)
                )
            }
        ),
        target: Target::Io,
    }
}
