use crate::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::codegen::generator::codec::sse::ty::delegate::{
    simple_delegate_decode, simple_delegate_encode,
};
use crate::codegen::generator::codec::sse::ty::*;
use crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::generator::codec::sse::lang::LangTrait;
use convert_case::{Case, Casing};

impl<'a> CodecSseTyTrait for RustOpaqueCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        Some(generate_generalized_rust_opaque_encode(
            lang,
            "null",
            MirType::RustOpaque(self.mir.clone()),
            self.context,
        ))
    }

    fn generate_decode(&self, lang: &Lang) -> Option<String> {
        Some(generate_generalized_rust_opaque_decode(
            lang,
            self.mir.clone().into(),
            self.mir.codec,
            self.context,
        ))
    }
}

const EXTERNAL_SIZE_TYPE: MirType = MirType::Primitive(MirTypePrimitive::I32);

pub(super) fn generate_generalized_rust_opaque_decode(
    lang: &Lang,
    mir: MirType,
    codec: RustOpaqueCodecMode,
    context: CodecSseTyContext,
) -> String {
    match lang {
        Lang::DartLang(_) => {
            format!(
                "return {}Impl.frbInternalSseDecode({}, {});",
                ApiDartGenerator::new(mir, context.as_api_dart_context()).dart_api_type(),
                lang.call_decode(&MirTypeRustOpaque::DELEGATE_TYPE),
                lang.call_decode(&EXTERNAL_SIZE_TYPE),
            )
        }
        Lang::RustLang(_) => simple_delegate_decode(
            lang,
            &MirTypeRustOpaque::DELEGATE_TYPE,
            &generate_decode_rust_opaque("inner", codec),
        ),
    }
}

pub(crate) fn generate_decode_rust_opaque(inner: &str, codec: RustOpaqueCodecMode) -> String {
    generate_maybe_unsafe(
        &format!(
            "decode_rust_opaque_{}({inner})",
            codec.to_string().to_case(Case::Snake)
        ),
        codec.needs_unsafe_block(),
    )
}

// TODO mv
pub(crate) fn generate_maybe_unsafe(inner: &str, needs_unsafe_block: bool) -> String {
    if needs_unsafe_block {
        format!("unsafe {{ {inner} }} ")
    } else {
        inner.to_owned()
    }
}

pub(super) fn generate_generalized_rust_opaque_encode(
    lang: &Lang,
    needs_move: &str,
    mir: MirType,
    context: CodecSseTyContext,
) -> String {
    match lang {
        Lang::DartLang(_) => simple_delegate_encode(
            lang,
            &MirTypeRustOpaque::DELEGATE_TYPE,
            &format!(
                "(self as {}Impl).frbInternalSseEncode(move: {needs_move})",
                ApiDartGenerator::new(mir, context.as_api_dart_context()).dart_api_type()
            ),
        ),
        Lang::RustLang(_) => {
            format!(
                "
                let (ptr, size) = self.sse_encode_raw();
                {};
                {};
                ",
                lang.call_encode(&MirTypeRustOpaque::DELEGATE_TYPE, "ptr"),
                lang.call_encode(&EXTERNAL_SIZE_TYPE, "size"),
            )
        }
    }
}
