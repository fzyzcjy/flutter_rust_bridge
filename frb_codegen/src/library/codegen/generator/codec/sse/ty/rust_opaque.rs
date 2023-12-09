use crate::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::codegen::generator::codec::sse::ty::delegate::{
    simple_delegate_decode, simple_delegate_encode,
};
use crate::codegen::generator::codec::sse::ty::*;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::generator::codec::sse::lang::LangTrait;

impl<'a> CodecSseTyTrait for RustOpaqueCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        Some(generate_generalized_rust_opaque_encode(lang, "null"))
    }

    fn generate_decode(&self, lang: &Lang) -> Option<String> {
        Some(generate_generalized_rust_opaque_decode(
            lang,
            self.ir.clone().into(),
            self.context,
        ))
    }
}

const EXTERNAL_SIZE_TYPE: IrType = IrType::Primitive(IrTypePrimitive::I32);

pub(super) fn generate_generalized_rust_opaque_decode(
    lang: &Lang,
    ir: IrType,
    context: CodecSseTyContext,
) -> String {
    match lang {
        Lang::DartLang(_) => {
            format!(
                "return {}.sseDecode({}, {});",
                ApiDartGenerator::new(ir, context.as_api_dart_context()).dart_api_type(),
                lang.call_decode(&IrTypeRustOpaque::DELEGATE_TYPE),
                lang.call_decode(&EXTERNAL_SIZE_TYPE),
            )
        }
        Lang::RustLang(_) => simple_delegate_decode(
            lang,
            &IrTypeRustOpaque::DELEGATE_TYPE,
            "flutter_rust_bridge::for_generated::sse_decode_rust_opaque(inner)",
        ),
    }
}

pub(super) fn generate_generalized_rust_opaque_encode(lang: &Lang, needs_move: &str) -> String {
    match lang {
        Lang::DartLang(_) => simple_delegate_encode(
            lang,
            &IrTypeRustOpaque::DELEGATE_TYPE,
            &format!("self.sseEncode(move: {needs_move})"),
        ),
        Lang::RustLang(_) => {
            format!(
                "
                let (ptr, size) = self.sse_encode_raw();
                {};
                {};
                ",
                lang.call_encode(&IrTypeRustOpaque::DELEGATE_TYPE, "ptr"),
                lang.call_encode(&EXTERNAL_SIZE_TYPE, "size"),
            )
        }
    }
}
