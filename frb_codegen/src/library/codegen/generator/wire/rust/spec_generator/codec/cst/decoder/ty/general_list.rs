use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::misc::{
    generate_class_from_fields, rust_wire_type_add_prefix_or_js_value,
};
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::{
    ExternFunc, ExternFuncParam,
};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::general_list::IrTypeGeneralList;
use crate::codegen::ir::ty::IrType::{Delegate, Optional};
use crate::codegen::ir::ty::{IrType, IrTypeTrait};

impl<'a> WireRustCodecCstGeneratorDecoderTrait for GeneralListWireRustCodecCstGenerator<'a> {
    fn generate_decoder_class(&self) -> Option<String> {
        Some(generate_class_from_fields(
            self.ir.clone(),
            self.context,
            &[
                format!(
                    "ptr: *mut {}{}",
                    general_list_maybe_extra_pointer_indirection(&self.ir),
                    WireRustCodecCstGenerator::new(self.ir.inner.clone(), self.context)
                        .rust_wire_type(Target::Io)
                ),
                "len: i32".to_string(),
            ],
        ))
    }

    fn generate_impl_decode_body(&self) -> Acc<Option<String>> {
        general_list_impl_decode_body()
    }

    fn generate_allocate_funcs(&self) -> Acc<WireRustOutputCode> {
        Acc {
            io: generate_list_generate_allocate_func(
                &self.ir.safe_ident(),
                &self.ir.clone().into(),
                &self.ir.inner,
                self.context,
            )
            .into(),
            ..Default::default()
        }
    }

    fn rust_wire_type(&self, target: Target) -> String {
        rust_wire_type_add_prefix_or_js_value(&self.ir, target)
    }

    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        target != Target::Wasm
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

pub(crate) fn general_list_impl_decode_body() -> Acc<Option<String>> {
    Acc {
        wasm: Some(DECODE_BODY_WASM.to_owned()),
        io: Some(DECODE_BODY_IO.to_owned()),
        ..Default::default()
    }
}

const DECODE_BODY_IO: &str = "
    let vec = unsafe {
        let wrap = flutter_rust_bridge::for_generated::box_from_leak_ptr(self);
        flutter_rust_bridge::for_generated::vec_from_leak_ptr(wrap.ptr, wrap.len)
    };
    vec.into_iter().map(CstDecodable::cst_decode).collect()
";
const DECODE_BODY_WASM: &str =
    "self.dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>().unwrap().iter().map(CstDecodable::cst_decode).collect()";

pub(crate) fn generate_list_generate_allocate_func(
    safe_ident: &str,
    list: &IrType,
    inner: &IrType,
    context: WireRustCodecCstGeneratorContext,
) -> ExternFunc {
    let list_generator = WireRustCodecCstGenerator::new(list.clone(), context);

    // let wasm = false;
    ExternFunc {
        func_name: format!("cst_new_{safe_ident}"),
        params: vec![ExternFuncParam {
            name: "len".to_owned(),
            rust_type: "i32".to_owned(),
            dart_type: "int".to_owned(),
        }],
        return_type: Some(
            [
                list_generator.rust_wire_modifier(Target::Io).as_str(),
                list_generator.rust_wire_type(Target::Io).as_str(),
            ]
                .concat(),
        ),
        body: format!(
            "let wrap = {} {{ ptr: flutter_rust_bridge::for_generated::new_leak_vec_ptr({}, len), len }};
                flutter_rust_bridge::for_generated::new_leak_box_ptr(wrap)",
            list_generator.rust_wire_type(Target::Io),
            if inner.is_primitive() {
                // A primitive enum list can use a default value since
                // `<i32>::new_with_null_ptr()` isn't implemented.
                "Default::default()".to_string()
            } else if matches!(inner, IrType::Optional(_) | IrType::RustOpaque(_) | IrType::DartOpaque(_)) {
                "core::ptr::null_mut()".to_string()
            } else {
                format!(
                    "<{}{}>::new_with_null_ptr()",
                    general_list_maybe_extra_pointer_indirection(&IrTypeGeneralList {
                        inner: Box::new(inner.clone())
                    }),
                    WireRustCodecCstGenerator::new(inner.clone(), context).rust_wire_type(Target::Io)
                )
            }
        ),
        target: Target::Io,
    }
}
