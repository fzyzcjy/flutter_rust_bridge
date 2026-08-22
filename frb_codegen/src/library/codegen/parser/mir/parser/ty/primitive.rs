use crate::codegen::ir::mir::ty::delegate::{
    MirTypeDelegate, MirTypeDelegateBigPrimitive, MirTypeDelegateCastedPrimitive,
};
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::ir::mir::ty::MirType::Primitive;
use crate::codegen::parser::mir::parser::ty::unencodable::SplayedSegment;
use crate::codegen::parser::mir::parser::ty::{TypeParserParsingContext, TypeParserWithContext};

impl TypeParserWithContext<'_, '_, '_> {
    pub(crate) fn parse_type_path_data_primitive(
        &mut self,
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<MirType>> {
        Ok(Some(match last_segment {
            // TODO: change to "if let guard" https://github.com/rust-lang/rust/issues/51114
            (name, []) if matches!(parse_primitive(name, self.context), Some(..)) => {
                parse_primitive(name, self.context).unwrap()
            }
            (name, []) if matches!(parse_big_primitive(name), Some(..)) => {
                parse_big_primitive(name).unwrap()
            }

            _ => return Ok(None),
        }))
    }
}

fn parse_primitive(s: &str, context: &TypeParserParsingContext) -> Option<MirType> {
    parse_primitive_raw(s).map(|primitive| transform_primitive(primitive, context))
}

fn parse_primitive_raw(s: &str) -> Option<MirTypePrimitive> {
    Some(match s {
        "u8" => MirTypePrimitive::U8,
        "i8" => MirTypePrimitive::I8,
        "u16" => MirTypePrimitive::U16,
        "i16" => MirTypePrimitive::I16,
        "u32" => MirTypePrimitive::U32,
        "i32" => MirTypePrimitive::I32,
        "u64" => MirTypePrimitive::U64,
        "i64" => MirTypePrimitive::I64,
        "f32" => MirTypePrimitive::F32,
        "f64" => MirTypePrimitive::F64,
        "bool" => MirTypePrimitive::Bool,
        "()" => MirTypePrimitive::Unit,
        "usize" => MirTypePrimitive::Usize,
        "isize" => MirTypePrimitive::Isize,
        _ => return None,
    })
}

fn transform_primitive(inner: MirTypePrimitive, context: &TypeParserParsingContext) -> MirType {
    if context.type_64bit_int
        || context.func_attributes.type_64bit_int()
        || (context.struct_or_enum_attributes.as_ref())
            .map(|x| x.type_64bit_int())
            .unwrap_or_default()
    {
        match inner {
            MirTypePrimitive::U64
            | MirTypePrimitive::I64
            | MirTypePrimitive::Usize
            | MirTypePrimitive::Isize => {
                return MirType::Delegate(MirTypeDelegate::CastedPrimitive(
                    MirTypeDelegateCastedPrimitive { inner },
                ))
            }
            _ => {}
        }
    }

    Primitive(inner)
}

fn parse_big_primitive(s: &str) -> Option<MirType> {
    Some(MirType::Delegate(MirTypeDelegate::BigPrimitive(match s {
        "i128" => MirTypeDelegateBigPrimitive::I128,
        "u128" => MirTypeDelegateBigPrimitive::U128,
        _ => return None,
    })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::codec::structs::CodecMode;
    use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
    use crate::codegen::parser::mir::ParseMode;
    use crate::utils::namespace::Namespace;

    fn context(type_64bit_int: bool) -> TypeParserParsingContext {
        TypeParserParsingContext {
            initiated_namespace: Namespace::default(),
            func_attributes: FrbAttributes::parse(&[]).unwrap(),
            struct_or_enum_attributes: None,
            rust_output_path_namespace: Namespace::default(),
            default_stream_sink_codec: CodecMode::Dco,
            default_rust_opaque_codec:
                crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode::Nom,
            owner: None,
            enable_lifetime: false,
            type_64bit_int,
            forbid_type_self: false,
            parse_mode: ParseMode::Early,
        }
    }

    /// Maps every directly supported primitive spelling and rejects unrelated names.
    #[test]
    fn parse_primitive_raw_maps_all_supported_names() {
        for (name, primitive) in [
            ("u8", MirTypePrimitive::U8),
            ("i8", MirTypePrimitive::I8),
            ("u16", MirTypePrimitive::U16),
            ("i16", MirTypePrimitive::I16),
            ("u32", MirTypePrimitive::U32),
            ("i32", MirTypePrimitive::I32),
            ("u64", MirTypePrimitive::U64),
            ("i64", MirTypePrimitive::I64),
            ("f32", MirTypePrimitive::F32),
            ("f64", MirTypePrimitive::F64),
            ("bool", MirTypePrimitive::Bool),
            ("()", MirTypePrimitive::Unit),
            ("usize", MirTypePrimitive::Usize),
            ("isize", MirTypePrimitive::Isize),
        ] {
            assert_eq!(parse_primitive_raw(name), Some(primitive));
        }
        assert_eq!(parse_primitive_raw("String"), None);
    }

    /// Keeps 128-bit primitives in the delegate representation.
    #[test]
    fn parse_big_primitive_recognizes_only_128_bit_integers() {
        for (name, primitive) in [
            ("i128", MirTypeDelegateBigPrimitive::I128),
            ("u128", MirTypeDelegateBigPrimitive::U128),
        ] {
            assert!(
                matches!(parse_big_primitive(name), Some(MirType::Delegate(MirTypeDelegate::BigPrimitive(actual))) if actual == primitive)
            );
        }
        assert!(parse_big_primitive("u64").is_none());
    }

    /// Casts 64-bit integer primitives only when the global flag is enabled.
    #[test]
    fn transform_primitive_applies_global_64_bit_casts() {
        let enabled = context(true);
        let disabled = context(false);

        for primitive in [
            MirTypePrimitive::I64,
            MirTypePrimitive::U64,
            MirTypePrimitive::Isize,
            MirTypePrimitive::Usize,
        ] {
            assert!(matches!(
                transform_primitive(primitive.clone(), &enabled),
                MirType::Delegate(MirTypeDelegate::CastedPrimitive(_))
            ));
            assert_eq!(
                transform_primitive(primitive.clone(), &disabled),
                MirType::Primitive(primitive)
            );
        }
        assert_eq!(
            transform_primitive(MirTypePrimitive::I32, &enabled),
            MirType::Primitive(MirTypePrimitive::I32)
        );
    }
}
