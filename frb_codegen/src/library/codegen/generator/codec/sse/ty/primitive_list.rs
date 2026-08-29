use crate::codegen::generator::codec::sse::ty::general_list::{
    general_list_generate_decode, general_list_generate_encode, list_len_method, LEN_TYPE,
};
use crate::codegen::generator::codec::sse::ty::primitive::get_serializer_dart_postfix;
use crate::codegen::generator::codec::sse::ty::*;
use crate::library::codegen::generator::codec::sse::lang::LangTrait;

impl CodecSseTyTrait for PrimitiveListCodecSseTy<'_> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        match lang {
            Lang::DartLang(_) => {
                let type_converter = if self.mir.strict_dart_type {
                    "self".to_owned()
                } else {
                    format!(
                        "self is {prim}List ? self : {prim}List.fromList(self)",
                        prim = get_serializer_dart_postfix(&self.mir.primitive, true),
                    )
                };

                Some(format!(
                    "{};
                    serializer.buffer.put{}List({type_converter});",
                    lang.call_encode(&LEN_TYPE, &format!("self.{}", list_len_method(lang))),
                    get_serializer_dart_postfix(&self.mir.primitive, true)
                ))
            }
            Lang::RustLang(_) => {
                // TODO do not use naive loop
                self.mir.strict_dart_type.then(|| {
                    general_list_generate_encode(
                        lang,
                        &MirType::Primitive(self.mir.primitive.clone()),
                    )
                })
            }
        }
    }

    fn generate_decode(&self, lang: &Lang) -> Option<String> {
        let var_decl = lang.var_decl();
        match lang {
            Lang::DartLang(_) => Some(format!(
                "{var_decl} len_ = {};
                return deserializer.buffer.get{}List(len_);",
                lang.call_decode(&LEN_TYPE),
                get_serializer_dart_postfix(&self.mir.primitive, true)
            )),
            Lang::RustLang(_) => {
                // TODO do not use naive loop
                self.mir.strict_dart_type.then(|| {
                    general_list_generate_decode(
                        lang,
                        &MirType::Primitive(self.mir.primitive.clone()),
                        self.context,
                    )
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
    use crate::codegen::generator::codec::sse::lang::{dart::DartLang, rust::RustLang};
    use std::collections::HashMap;
    use std::path::PathBuf;

    fn context<'a>(
        pack: &'a MirPack,
        config: &'a GeneratorApiDartInternalConfig,
    ) -> CodecSseTyContext<'a> {
        CodecSseTyContext::new(pack, config)
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

    /// Uses direct typed lists for strict Dart types and converts loose inputs.
    #[test]
    fn dart_primitive_list_encode_covers_strict_and_loose_inputs() {
        let pack = pack();
        let config = config();
        let lang = Lang::DartLang(DartLang);
        for (strict, expected) in [
            (true, "putInt32List(self)"),
            (false, "Int32List.fromList(self)"),
        ] {
            let generator = PrimitiveListCodecSseTy::new(
                MirTypePrimitiveList {
                    primitive: MirTypePrimitive::I32,
                    strict_dart_type: strict,
                },
                context(&pack, &config),
            );
            assert!(generator.generate_encode(&lang).unwrap().contains(expected));
            assert!(generator
                .generate_decode(&lang)
                .unwrap()
                .contains("getInt32List(len_)"));
        }
    }

    /// Suppresses loose Rust lists and delegates strict Rust lists element by element.
    #[test]
    fn rust_primitive_list_encode_and_decode_cover_strictness_matrix() {
        let pack = pack();
        let config = config();
        let lang = Lang::RustLang(RustLang);
        for strict in [false, true] {
            let generator = PrimitiveListCodecSseTy::new(
                MirTypePrimitiveList {
                    primitive: MirTypePrimitive::I32,
                    strict_dart_type: strict,
                },
                context(&pack, &config),
            );
            assert_eq!(generator.generate_encode(&lang).is_some(), strict);
            assert_eq!(generator.generate_decode(&lang).is_some(), strict);
        }
    }
}
