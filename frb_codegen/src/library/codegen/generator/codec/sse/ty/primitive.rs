use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for PrimitiveCodecSseTy<'a> {
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
