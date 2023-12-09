use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for PrimitiveCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        let dart_cast = match self.ir {
            IrTypePrimitive::Bool => " ? 1 : 0",
            _ => "",
        };
        let rust_cast = match self.ir {
            IrTypePrimitive::Bool | IrTypePrimitive::Usize | IrTypePrimitive::Isize => " as _",
            _ => "",
        };

        Some(match self.ir {
            IrTypePrimitive::Unit => "".into(),
            _ => match lang {
                Lang::DartLang(_) => format!(
                    "serializer.buffer.put{}(self{dart_cast});",
                    get_serializer_dart_postfix(&self.ir)
                ),
                Lang::RustLang(_) => format!(
                    "serializer.cursor.write_{}{}(self{rust_cast}).unwrap();",
                    get_serializer_rust_type(&self.ir),
                    maybe_endian(&self.ir),
                ),
            },
        })
    }

    fn generate_decode(&self, lang: &Lang) -> Option<String> {
        let dart_cast = match self.ir {
            IrTypePrimitive::Bool => " ? 1 : 0",
            _ => "",
        };
        let rust_cast = match self.ir {
            IrTypePrimitive::Bool | IrTypePrimitive::Usize | IrTypePrimitive::Isize => " as _",
            _ => "",
        };

        Some(match self.ir {
            IrTypePrimitive::Unit => "".into(),
            _ => match lang {
                Lang::DartLang(_) => format!(
                    "return deserializer.buffer.get{}(){dart_cast};",
                    get_serializer_dart_postfix(&self.ir)
                ),
                Lang::RustLang(_) => {
                    format!(
                        "deserializer.cursor.read_{}{}().unwrap(){rust_cast}",
                        get_serializer_rust_type(&self.ir),
                        maybe_endian(&self.ir),
                    )
                }
            },
        })
    }
}

pub(super) fn get_serializer_dart_postfix(prim: &IrTypePrimitive) -> &'static str {
    match prim {
        IrTypePrimitive::U8 => "Uint8",
        IrTypePrimitive::I8 => "Int8",
        IrTypePrimitive::U16 => "Uint16",
        IrTypePrimitive::I16 => "Int16",
        IrTypePrimitive::U32 => "Uint32",
        IrTypePrimitive::I32 => "Int32",
        IrTypePrimitive::U64 => "Uint64",
        IrTypePrimitive::I64 => "Int64",
        IrTypePrimitive::Usize => "Uint64",
        IrTypePrimitive::Isize => "Int64",
        IrTypePrimitive::F32 => "Float32",
        IrTypePrimitive::F64 => "Float64",
        IrTypePrimitive::Bool => "Uint8",
        IrTypePrimitive::Unit => unreachable!(),
    }
}

pub(super) fn get_serializer_rust_type(prim: &IrTypePrimitive) -> String {
    match prim {
        // TODO make it adapt to 32/64bit platform
        IrTypePrimitive::Usize => "u64".to_owned(),
        IrTypePrimitive::Isize => "i64".to_owned(),
        IrTypePrimitive::bool => "u8".to_owned(),
        _ => prim.rust_api_type(),
    }
}

fn maybe_endian(ty: &IrTypePrimitive) -> &'static str {
    match ty {
        IrTypePrimitive::U8 | IrTypePrimitive::I8 => "",
        _ => "::<NativeEndian>",
    }
}
