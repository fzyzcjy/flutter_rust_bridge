use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::misc::{
    generate_class_from_fields, rust_wire_type_add_prefix_or_js_value,
};
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::{
    ExternFunc, ExternFuncParam,
};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::mir::ty::delegate::MirTypeDelegate;
use crate::codegen::ir::mir::ty::general_list::MirTypeGeneralList;
use crate::codegen::ir::mir::ty::MirType::{Delegate, Optional};
use crate::codegen::ir::mir::ty::{MirType, MirTypeTrait};

impl<'a> WireRustCodecCstGeneratorDecoderTrait for GeneralListWireRustCodecCstGenerator<'a> {
    fn generate_decoder_class(&self) -> Option<WireRustOutputCode> {
        Some(generate_class_from_fields(
            self.mir.clone(),
            self.context,
            &[
                format!(
                    "ptr: *mut {}{}",
                    general_list_maybe_extra_pointer_indirection(&self.mir),
                    WireRustCodecCstGenerator::new(self.mir.inner.clone(), self.context)
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
                &self.mir.safe_ident(),
                &self.mir.clone().into(),
                &self.mir.inner,
                self.context,
            )
            .into(),
            ..Default::default()
        }
    }

    fn rust_wire_type(&self, target: Target) -> String {
        rust_wire_type_add_prefix_or_js_value(&self.mir, target)
    }

    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        target != Target::Web
    }
}

/// Does it need additional indirection for types put behind a vector
fn general_list_maybe_extra_pointer_indirection(mir: &MirTypeGeneralList) -> &'static str {
    if matches!(
        *mir.inner,
        Optional(_)
            | Delegate(MirTypeDelegate::String)
            | Delegate(MirTypeDelegate::StreamSink(_))
            | Delegate(MirTypeDelegate::Uuid)
            | MirType::PrimitiveList(_)
    ) {
        "*mut "
    } else {
        ""
    }
}

pub(crate) fn general_list_impl_decode_body() -> Acc<Option<String>> {
    Acc {
        web: Some(DECODE_BODY_WEB.to_owned()),
        io: Some(DECODE_BODY_IO.to_owned()),
        ..Default::default()
    }
}

const DECODE_BODY_IO: &str = "
    let vec = unsafe {
        let wrap = flutter_rust_bridge::for_generated::box_from_leak_ptr(self);
        flutter_rust_bridge::for_generated::vec_from_leak_ptr(wrap.ptr, wrap.len)
    };
    vec.into_iter().map(CstDecode::cst_decode).collect()
";
const DECODE_BODY_WEB: &str =
    "self.dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>().unwrap().iter().map(CstDecode::cst_decode).collect()";

pub(crate) fn generate_list_generate_allocate_func(
    safe_ident: &str,
    list: &MirType,
    inner: &MirType,
    context: WireRustCodecCstGeneratorContext,
) -> ExternFunc {
    let list_generator = WireRustCodecCstGenerator::new(list.clone(), context);

    // let web = false;
    ExternFunc {
        partial_func_name: format!("cst_new_{safe_ident}"),
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
            if inner.is_primitive() || matches!(inner, MirType::RustOpaque(_)) || matches!(inner, MirType::Delegate(MirTypeDelegate::RustAutoOpaqueExplicit(_))) || matches!(inner, MirType::RustAutoOpaqueImplicit(_)) {
                // A primitive enum list can use a default value since
                // `<i32>::new_with_null_ptr()` isn't implemented.
                "Default::default()".to_string()
            } else if matches!(inner, MirType::Optional(_) | MirType::DartOpaque(_)) {
                "core::ptr::null_mut()".to_string()
            } else {
                format!(
                    "<{}{}>::new_with_null_ptr()",
                    general_list_maybe_extra_pointer_indirection(&MirTypeGeneralList {
                        inner: Box::new(inner.clone())
                    }),
                    WireRustCodecCstGenerator::new(inner.clone(), context).rust_wire_type(Target::Io)
                )
            }
        ),
        target: Target::Io,
        needs_ffigen: true,
    }
}
