use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::misc::{
    generate_class_from_fields, JS_VALUE,
};
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::{
    ExternFunc, ExternFuncParam,
};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::ir::mir::ty::primitive_list::MirTypePrimitiveList;
use crate::codegen::ir::mir::ty::MirTypeTrait;

impl<'a> WireRustCodecCstGeneratorDecoderTrait for PrimitiveListWireRustCodecCstGenerator<'a> {
    fn generate_decoder_class(&self) -> Option<WireRustOutputCode> {
        Some(generate_class_from_fields(
            self.mir.clone(),
            self.context,
            &[
                format!(
                    "ptr: *mut {}",
                    WireRustCodecCstGenerator::new(self.mir.primitive.clone(), self.context)
                        .rust_wire_type(Target::Io)
                ),
                "len: i32".to_string(),
            ],
        ))
    }

    fn generate_impl_decode_body(&self) -> Acc<Option<String>> {
        Acc {
            // for non-strict case, let the strict counterpart generate code
            web: self.mir.strict_dart_type.then(|| "self.into_vec()".into()),
            io: Some(
                "unsafe {
                    let wrap = flutter_rust_bridge::for_generated::box_from_leak_ptr(self);
                    flutter_rust_bridge::for_generated::vec_from_leak_ptr(wrap.ptr, wrap.len)
                }"
                .into(),
            ),
            ..Default::default()
        }
    }

    fn generate_impl_decode_jsvalue_body(&self) -> Option<std::borrow::Cow<str>> {
        if !self.mir.strict_dart_type {
            return None;
        }

        match self.mir.primitive {
            // frb-coverage:ignore-start
            MirTypePrimitive::Bool | MirTypePrimitive::Unit => Some("todo!()".into()),
            // frb-coverage:ignore-end
            MirTypePrimitive::I64 | MirTypePrimitive::U64 => Some(
                format!(
                    "let buf = self.dyn_into::<{}>().unwrap();
                    let buf = flutter_rust_bridge::for_generated::js_sys::Uint8Array::new(&buf.buffer());
                    flutter_rust_bridge::for_generated::slice_from_byte_buffer(buf.to_vec()).into()",
                    rust_web_wire_type(&self.mir)
                )
                    .into(),
            ),
            _ => Some(
                format!(
                    "self.unchecked_into::<{}>().to_vec().into()",
                    rust_web_wire_type(&self.mir)
                )
                    .into(),
            ),
        }
    }

    fn generate_allocate_funcs(&self) -> Acc<WireRustOutputCode> {
        Acc {
            io: ExternFunc {
                partial_func_name: format!("cst_new_{}", self.mir.safe_ident()),
                params: vec![ExternFuncParam {
                    name: "len".to_owned(),
                    rust_type: "i32".to_owned(),
                    dart_type: "int".to_owned(),
                }],
                return_type: Some(format!(
                    "{}{}",
                    self.rust_wire_modifier(Target::Io),
                    self.rust_wire_type(Target::Io)
                )),
                body: format!(
                    "let ans = {} {{ ptr: flutter_rust_bridge::for_generated::new_leak_vec_ptr(Default::default(), len), len }};
                    flutter_rust_bridge::for_generated::new_leak_box_ptr(ans)",
                    self.rust_wire_type(Target::Io),
                ),
                target: Target::Io,
                needs_ffigen: true,
            }.into(),
            ..Default::default()
        }
    }

    fn rust_wire_type(&self, target: Target) -> String {
        if let Target::Web = target {
            match self.mir.primitive {
                MirTypePrimitive::Bool | MirTypePrimitive::Unit => JS_VALUE.into(),
                _ => format!("Box<[{}]>", self.mir.primitive.rust_api_type()),
            }
        } else {
            format!("wire_cst_{}", self.mir.safe_ident())
        }
    }

    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        target != Target::Web
    }
}

fn rust_web_wire_type(mir: &MirTypePrimitiveList) -> &str {
    match &mir.primitive {
        MirTypePrimitive::U8 => "flutter_rust_bridge::for_generated::js_sys::Uint8Array",
        MirTypePrimitive::I8 => "flutter_rust_bridge::for_generated::js_sys::Int8Array",
        MirTypePrimitive::U16 => "flutter_rust_bridge::for_generated::js_sys::Uint16Array",
        MirTypePrimitive::I16 => "flutter_rust_bridge::for_generated::js_sys::Int16Array",
        MirTypePrimitive::U32 | MirTypePrimitive::Usize => {
            "flutter_rust_bridge::for_generated::js_sys::Uint32Array"
        }
        MirTypePrimitive::I32 | MirTypePrimitive::Isize => {
            "flutter_rust_bridge::for_generated::js_sys::Int32Array"
        }
        MirTypePrimitive::U64 => "flutter_rust_bridge::for_generated::js_sys::BigUint64Array",
        MirTypePrimitive::I64 => "flutter_rust_bridge::for_generated::js_sys::BigInt64Array",
        MirTypePrimitive::F32 => "flutter_rust_bridge::for_generated::js_sys::Float32Array",
        MirTypePrimitive::F64 => "flutter_rust_bridge::for_generated::js_sys::Float64Array",
        MirTypePrimitive::Bool | MirTypePrimitive::Unit => {
            "flutter_rust_bridge::for_generated::js_sys::Array"
        }
    }
}
