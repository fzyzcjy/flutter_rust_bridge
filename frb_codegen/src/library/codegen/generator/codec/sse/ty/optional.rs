use crate::codegen::generator::codec::sse::ty::*;
use crate::library::codegen::generator::codec::sse::lang::LangTrait;

impl CodecSseTyTrait for OptionalCodecSseTy<'_> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        let self_is_not_null = match lang {
            Lang::DartLang(_) => "self != null",
            Lang::RustLang(_) => "self.is_some()",
        };
        let encode_flag = lang.call_encode(&Primitive(MirTypePrimitive::Bool), self_is_not_null);

        Some(match lang {
            Lang::DartLang(_) => format!(
                "
                {encode_flag};
                if (self != null) {{
                    {};
                }}
                ",
                lang.call_encode(&self.mir.inner, "self"),
            ),
            Lang::RustLang(_) => format!(
                "
                {encode_flag};
                if let Some(value) = self {{
                    {};
                }}
                ",
                lang.call_encode(&self.mir.inner, "value"),
            ),
        })
    }

    fn generate_decode(&self, lang: &Lang) -> Option<String> {
        let wrapper = match lang {
            Lang::DartLang(_) => "",
            Lang::RustLang(_) => "Some",
        };
        let null = match lang {
            Lang::DartLang(_) => "null",
            Lang::RustLang(_) => "None",
        };

        Some(format!(
            "
            if ({}) {{
                return {wrapper}({});
            }} else {{
                return {null};
            }}
            ",
            lang.call_decode(&Primitive(MirTypePrimitive::Bool)),
            lang.call_decode(&self.mir.inner),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
    use crate::codegen::generator::codec::sse::lang::{dart::DartLang, rust::RustLang};
    use std::collections::HashMap;
    use std::path::PathBuf;

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

    /// Emits presence checks and inner calls using Dart optional syntax.
    #[test]
    fn dart_optional_encode_and_decode_use_null_and_inner_codec_calls() {
        let pack = pack();
        let config = config();
        let generator = OptionalCodecSseTy::new(
            MirTypeOptional::new(Primitive(MirTypePrimitive::I32)),
            CodecSseTyContext::new(&pack, &config),
        );
        let lang = Lang::DartLang(DartLang);

        assert!(generator
            .generate_encode(&lang)
            .unwrap()
            .contains("self != null"));
        assert!(generator
            .generate_encode(&lang)
            .unwrap()
            .contains("sse_encode_i_32(self, serializer)"));
        assert!(generator
            .generate_decode(&lang)
            .unwrap()
            .contains("sse_decode_i_32(deserializer)"));
        assert!(generator
            .generate_decode(&lang)
            .unwrap()
            .contains("return null;"));
    }

    /// Emits presence checks and inner calls using Rust option syntax.
    #[test]
    fn rust_optional_encode_and_decode_use_option_and_inner_codec_calls() {
        let pack = pack();
        let config = config();
        let generator = OptionalCodecSseTy::new(
            MirTypeOptional::new(Primitive(MirTypePrimitive::I32)),
            CodecSseTyContext::new(&pack, &config),
        );
        let lang = Lang::RustLang(RustLang);

        assert!(generator
            .generate_encode(&lang)
            .unwrap()
            .contains("self.is_some()"));
        assert!(generator
            .generate_encode(&lang)
            .unwrap()
            .contains("<i32>::sse_encode(value, serializer)"));
        assert!(generator
            .generate_decode(&lang)
            .unwrap()
            .contains("return Some(<i32>::sse_decode(deserializer));"));
        assert!(generator
            .generate_decode(&lang)
            .unwrap()
            .contains("return None;"));
    }
}
