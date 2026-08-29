use crate::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::codegen::generator::codec::sse::ty::delegate::{
    simple_delegate_decode, simple_delegate_encode,
};
use crate::codegen::generator::codec::sse::ty::*;
use crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::generator::codec::sse::lang::LangTrait;
use convert_case::{Case, Casing};

impl CodecSseTyTrait for RustOpaqueCodecSseTy<'_> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
    use crate::codegen::generator::codec::sse::lang::rust::RustLang;
    use std::collections::HashMap;
    use std::path::PathBuf;

    fn pack() -> MirPack {
        MirPack {
            funcs_all: vec![],
            extra_types_all: vec![],
            struct_pool: Default::default(),
            enum_pool: Default::default(),
            dart_code_of_type: Default::default(),
            existing_handler: None,
            skips: vec![],
            trait_impls: vec![],
            extra_rust_output_code: String::new(),
            extra_dart_output_code: Default::default(),
        }
    }

    fn config() -> GeneratorApiDartInternalConfig {
        GeneratorApiDartInternalConfig {
            dart_collection_deep_equality: false,
            dart_enums_style: true,
            dart3: true,
            dart_decl_base_output_path: PathBuf::new(),
            dart_impl_output_path: Default::default(),
            dart_entrypoint_class_name: "Entrypoint".into(),
            dart_preamble: String::new(),
            dart_type_rename: HashMap::new(),
        }
    }

    /// Emits codec-specific Rust opaque decode calls with the required safety boundary.
    #[test]
    fn rust_opaque_decode_uses_codec_specific_safety() {
        assert_eq!(
            generate_decode_rust_opaque("inner", RustOpaqueCodecMode::Nom),
            "unsafe { decode_rust_opaque_nom(inner) } "
        );
        assert_eq!(
            generate_decode_rust_opaque("inner", RustOpaqueCodecMode::Moi),
            "decode_rust_opaque_moi(inner)"
        );
    }

    /// Preserves expressions unless the caller explicitly requires an unsafe block.
    #[test]
    fn maybe_unsafe_wraps_only_unsafe_expressions() {
        assert_eq!(
            generate_maybe_unsafe("decode(value)", false),
            "decode(value)"
        );
        assert_eq!(
            generate_maybe_unsafe("decode(value)", true),
            "unsafe { decode(value) } "
        );
    }

    /// Emits Rust opaque pointer-size encoding and delegated decoding for both codecs.
    #[test]
    fn generalized_rust_opaque_paths_emit_rust_encode_and_decode_bodies() {
        let pack = pack();
        let config = config();
        let context = CodecSseTyContext::new(&pack, &config);
        let lang = Lang::RustLang(RustLang);
        let encode = generate_generalized_rust_opaque_encode(
            &lang,
            "ignored",
            Primitive(MirTypePrimitive::I32),
            context,
        );
        assert!(encode.contains("self.sse_encode_raw()"));
        assert!(encode.contains("<usize>::sse_encode(ptr, serializer)"));
        assert!(encode.contains("<i32>::sse_encode(size, serializer)"));
        let decode = generate_generalized_rust_opaque_decode(
            &lang,
            Primitive(MirTypePrimitive::I32),
            RustOpaqueCodecMode::Nom,
            context,
        );
        assert!(decode.contains("<usize>::sse_decode(deserializer)"));
        assert!(decode.contains("unsafe { decode_rust_opaque_nom(inner) }"));
    }
}
