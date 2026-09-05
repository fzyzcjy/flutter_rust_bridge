use crate::codegen::generator::codec::sse::ty::*;

impl CodecSseTyTrait for PrimitiveCodecSseTy<'_> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        let dart_cast = match self.mir {
            MirTypePrimitive::Bool => " ? 1 : 0",
            _ => "",
        };
        let rust_cast = match self.mir {
            MirTypePrimitive::Bool | MirTypePrimitive::Usize | MirTypePrimitive::Isize => " as _",
            _ => "",
        };

        Some(match self.mir {
            MirTypePrimitive::Unit => "".into(),
            _ => match lang {
                Lang::DartLang(_) => format!(
                    "serializer.buffer.put{}(self{dart_cast});",
                    get_serializer_dart_postfix(&self.mir, false)
                ),
                Lang::RustLang(_) => format!(
                    "serializer.cursor.write_{}{}(self{rust_cast}).unwrap();",
                    get_serializer_rust_type(&self.mir),
                    maybe_endian(&self.mir),
                ),
            },
        })
    }

    fn generate_decode(&self, lang: &Lang) -> Option<String> {
        let dart_cast = match self.mir {
            MirTypePrimitive::Bool => " != 0",
            _ => "",
        };
        let rust_cast = match self.mir {
            MirTypePrimitive::Bool => " != 0",
            MirTypePrimitive::Usize | MirTypePrimitive::Isize => " as _",
            _ => "",
        };

        Some(match self.mir {
            MirTypePrimitive::Unit => "".into(),
            _ => match lang {
                Lang::DartLang(_) => format!(
                    "return deserializer.buffer.get{}(){dart_cast};",
                    get_serializer_dart_postfix(&self.mir, false)
                ),
                Lang::RustLang(_) => {
                    format!(
                        "deserializer.cursor.read_{}{}().unwrap(){rust_cast}",
                        get_serializer_rust_type(&self.mir),
                        maybe_endian(&self.mir),
                    )
                }
            },
        })
    }
}

pub(super) fn get_serializer_dart_postfix(
    prim: &MirTypePrimitive,
    mode_list: bool,
) -> &'static str {
    match prim {
        MirTypePrimitive::U8 => "Uint8",
        MirTypePrimitive::I8 => "Int8",
        MirTypePrimitive::U16 => "Uint16",
        MirTypePrimitive::I16 => "Int16",
        MirTypePrimitive::U32 => "Uint32",
        MirTypePrimitive::I32 => "Int32",
        MirTypePrimitive::I64 | MirTypePrimitive::Isize => {
            if mode_list {
                "Int64"
            } else {
                "PlatformInt64"
            }
        }
        MirTypePrimitive::U64 | MirTypePrimitive::Usize => {
            if mode_list {
                "Uint64"
            } else {
                "BigUint64"
            }
        }
        MirTypePrimitive::F32 => "Float32",
        MirTypePrimitive::F64 => "Float64",
        MirTypePrimitive::Bool => "Uint8",
        // frb-coverage:ignore-start
        MirTypePrimitive::Unit => unreachable!(),
        // frb-coverage:ignore-end
    }
}

pub(super) fn get_serializer_rust_type(prim: &MirTypePrimitive) -> String {
    match prim {
        // TODO make it adapt to 32/64bit platform
        MirTypePrimitive::Usize => "u64".to_owned(),
        MirTypePrimitive::Isize => "i64".to_owned(),
        MirTypePrimitive::Bool => "u8".to_owned(),
        _ => prim.rust_api_type(),
    }
}

fn maybe_endian(ty: &MirTypePrimitive) -> &'static str {
    match ty {
        MirTypePrimitive::U8 | MirTypePrimitive::I8 | MirTypePrimitive::Bool => "",
        _ => "::<NativeEndian>",
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

    /// Maps every primitive to its Dart scalar and list buffer suffixes.
    #[test]
    fn serializer_dart_postfixes_cover_scalar_and_list_special_cases() {
        let cases = [
            (MirTypePrimitive::U8, "Uint8", "Uint8"),
            (MirTypePrimitive::I8, "Int8", "Int8"),
            (MirTypePrimitive::U16, "Uint16", "Uint16"),
            (MirTypePrimitive::I16, "Int16", "Int16"),
            (MirTypePrimitive::U32, "Uint32", "Uint32"),
            (MirTypePrimitive::I32, "Int32", "Int32"),
            (MirTypePrimitive::I64, "PlatformInt64", "Int64"),
            (MirTypePrimitive::U64, "BigUint64", "Uint64"),
            (MirTypePrimitive::F32, "Float32", "Float32"),
            (MirTypePrimitive::F64, "Float64", "Float64"),
            (MirTypePrimitive::Bool, "Uint8", "Uint8"),
            (MirTypePrimitive::Usize, "BigUint64", "Uint64"),
            (MirTypePrimitive::Isize, "PlatformInt64", "Int64"),
        ];

        for (primitive, scalar, list) in cases {
            assert_eq!(get_serializer_dart_postfix(&primitive, false), scalar);
            assert_eq!(get_serializer_dart_postfix(&primitive, true), list);
        }
    }

    /// Normalizes platform integers and booleans to their Rust wire types.
    #[test]
    fn serializer_rust_types_and_endianness_cover_wire_special_cases() {
        assert_eq!(get_serializer_rust_type(&MirTypePrimitive::Usize), "u64");
        assert_eq!(get_serializer_rust_type(&MirTypePrimitive::Isize), "i64");
        assert_eq!(get_serializer_rust_type(&MirTypePrimitive::Bool), "u8");
        assert_eq!(get_serializer_rust_type(&MirTypePrimitive::I32), "i32");

        for primitive in [
            MirTypePrimitive::U8,
            MirTypePrimitive::I8,
            MirTypePrimitive::Bool,
        ] {
            assert_eq!(maybe_endian(&primitive), "");
        }
        for primitive in [
            MirTypePrimitive::U16,
            MirTypePrimitive::I64,
            MirTypePrimitive::F32,
            MirTypePrimitive::Usize,
        ] {
            assert_eq!(maybe_endian(&primitive), "::<NativeEndian>");
        }
    }

    /// Emits unit suppression and Dart boolean conversions for both codec directions.
    #[test]
    fn primitive_generator_handles_unit_and_dart_boolean_branches() {
        let pack = pack();
        let config = config();
        let context = context(&pack, &config);
        let unit = PrimitiveCodecSseTy::new(MirTypePrimitive::Unit, context);
        assert_eq!(
            unit.generate_encode(&Lang::DartLang(DartLang)),
            Some(String::new())
        );
        assert_eq!(
            unit.generate_decode(&Lang::RustLang(RustLang)),
            Some(String::new())
        );
        let boolean = PrimitiveCodecSseTy::new(MirTypePrimitive::Bool, context);
        assert_eq!(
            boolean.generate_encode(&Lang::DartLang(DartLang)),
            Some("serializer.buffer.putUint8(self ? 1 : 0);".into())
        );
        assert_eq!(
            boolean.generate_decode(&Lang::DartLang(DartLang)),
            Some("return deserializer.buffer.getUint8() != 0;".into())
        );
    }

    /// Emits Rust boolean and platform-integer casts with the required endian form.
    #[test]
    fn primitive_generator_handles_rust_boolean_and_platform_integer_branches() {
        let pack = pack();
        let config = config();
        let context = context(&pack, &config);
        let lang = Lang::RustLang(RustLang);
        let boolean = PrimitiveCodecSseTy::new(MirTypePrimitive::Bool, context);
        assert_eq!(
            boolean.generate_encode(&lang),
            Some("serializer.cursor.write_u8(self as _).unwrap();".into())
        );
        assert_eq!(
            boolean.generate_decode(&lang),
            Some("deserializer.cursor.read_u8().unwrap() != 0".into())
        );
        for (primitive, wire) in [
            (MirTypePrimitive::Usize, "u64"),
            (MirTypePrimitive::Isize, "i64"),
        ] {
            let generator = PrimitiveCodecSseTy::new(primitive, context);
            assert!(generator
                .generate_encode(&lang)
                .unwrap()
                .contains(&format!("write_{wire}::<NativeEndian>(self as _)")));
            assert!(generator
                .generate_decode(&lang)
                .unwrap()
                .contains(&format!("read_{wire}::<NativeEndian>().unwrap() as _")));
        }
    }
}
