use crate::codegen::ir::mir::ty::delegate::MirTypeDelegate;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::ir::mir::ty::MirType::{EnumRef, StructRef};
use crate::codegen::parser::mir::parser::ty::unencodable::splay_segments;
use crate::codegen::parser::mir::parser::ty::{TypeParser, TypeParserParsingContext};
use anyhow::Context;

#[allow(clippy::single_match)] // deliberate do so to ensure style consistency
pub(crate) fn parse_type_maybe_result(
    mir: &MirType,
    type_parser: &mut TypeParser,
    context: &TypeParserParsingContext,
) -> anyhow::Result<ResultTypeInfo> {
    if let MirType::RustAutoOpaqueImplicit(inner) = mir {
        match splay_segments(&inner.raw.segments).last() {
            Some(("Result", args)) => {
                return parse_type_result(
                    &(args.iter())
                        .map(|arg| type_parser.parse_type(arg, context))
                        .collect::<anyhow::Result<Vec<_>>>()?,
                );
            }
            _ => {}
        }
    }

    Ok(ResultTypeInfo {
        ok_output: mir.clone(),
        error_output: None,
    })
}

fn parse_type_result(args: &[MirType]) -> anyhow::Result<ResultTypeInfo> {
    let ok_output = args
        .first()
        .with_context(|| "invalid number of args".to_string())?;

    let is_anyhow = args.len() == 1
        || args.iter().any(|x| {
            if let MirType::RustAutoOpaqueImplicit(inner) = x {
                // Indeed `anyhow :: Error`, but we stripped the prefixes
                return inner.raw.string.with_static_lifetime().trim() == "Error";
            }
            false
        });

    let error_output = if is_anyhow {
        Some(MirType::Delegate(MirTypeDelegate::AnyhowException))
    } else {
        args.last().cloned()
    };

    let error_output = error_output.map(set_is_exception_flag);

    Ok(ResultTypeInfo {
        ok_output: ok_output.clone(),
        error_output,
    })
}

pub(crate) struct ResultTypeInfo {
    pub ok_output: MirType,
    pub error_output: Option<MirType>,
}

fn set_is_exception_flag(mut ty: MirType) -> MirType {
    match &mut ty {
        StructRef(ref mut inner) => {
            inner.is_exception = true;
        }
        EnumRef(ref mut inner) => {
            inner.is_exception = true;
        }
        _ => {}
    }
    ty
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::ir::mir::func::OwnershipMode;
    use crate::codegen::ir::mir::llfetime_aware_type::MirLifetimeAwareType;
    use crate::codegen::ir::mir::ty::enumeration::{MirEnumIdent, MirTypeEnumRef};
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use crate::codegen::ir::mir::ty::rust_auto_opaque_implicit::{
        MirRustAutoOpaqueRaw, MirTypeRustAutoOpaqueImplicit,
    };
    use crate::codegen::ir::mir::ty::rust_opaque::{
        MirRustOpaqueInner, MirTypeRustOpaque, RustOpaqueCodecMode,
    };
    use crate::codegen::ir::mir::ty::structure::{MirStructIdent, MirTypeStructRef};
    use crate::utils::namespace::{Namespace, NamespacedName};

    fn implicit_error_type() -> MirType {
        MirType::RustAutoOpaqueImplicit(MirTypeRustAutoOpaqueImplicit {
            ownership_mode: OwnershipMode::Owned,
            raw: MirRustAutoOpaqueRaw {
                string: MirLifetimeAwareType::new("Error".to_owned()),
                segments: vec![],
            },
            inner: MirTypeRustOpaque {
                namespace: Namespace::default(),
                inner: MirRustOpaqueInner(MirLifetimeAwareType::new("Error".to_owned())),
                codec: RustOpaqueCodecMode::Nom,
                dart_api_type: None,
                brief_name: true,
            },
            reason: None,
            ignore: false,
        })
    }

    /// Maps a single Result argument to the anyhow exception delegate.
    #[test]
    fn parse_type_result_treats_single_argument_as_anyhow() {
        let output = parse_type_result(&[MirType::Primitive(MirTypePrimitive::U8)]).unwrap();

        assert_eq!(output.ok_output, MirType::Primitive(MirTypePrimitive::U8));
        assert!(matches!(
            output.error_output,
            Some(MirType::Delegate(MirTypeDelegate::AnyhowException))
        ));
    }

    /// Retains a concrete error type when Result has two non-anyhow arguments.
    #[test]
    fn parse_type_result_uses_the_last_concrete_argument_as_error() {
        let output = parse_type_result(&[
            MirType::Primitive(MirTypePrimitive::U8),
            MirType::Primitive(MirTypePrimitive::I32),
        ])
        .unwrap();

        assert_eq!(output.ok_output, MirType::Primitive(MirTypePrimitive::U8));
        assert!(matches!(
            output.error_output,
            Some(MirType::Primitive(MirTypePrimitive::I32))
        ));
    }

    /// Rejects a Result type that has no success argument.
    #[test]
    fn parse_type_result_rejects_empty_arguments() {
        assert!(parse_type_result(&[]).is_err());
    }

    /// Detects an implicit opaque anyhow Error among Result arguments.
    #[test]
    fn parse_type_result_detects_implicit_anyhow_error() {
        let output = parse_type_result(&[
            MirType::Primitive(MirTypePrimitive::U8),
            implicit_error_type(),
        ])
        .unwrap();

        assert_eq!(output.ok_output, MirType::Primitive(MirTypePrimitive::U8));
        assert!(matches!(
            output.error_output,
            Some(MirType::Delegate(MirTypeDelegate::AnyhowException))
        ));
    }

    /// Marks struct and enum Result errors as exceptions without changing other types.
    #[test]
    fn set_is_exception_flag_marks_struct_and_enum_references() {
        let namespace = Namespace::new(vec!["crate".to_owned()]);
        let struct_type = MirType::StructRef(MirTypeStructRef {
            ident: MirStructIdent(NamespacedName::new(
                namespace.clone(),
                "StructError".to_owned(),
            )),
            is_exception: false,
        });
        let enum_type = MirType::EnumRef(MirTypeEnumRef {
            ident: MirEnumIdent(NamespacedName::new(namespace, "EnumError".to_owned())),
            is_exception: false,
        });

        assert!(matches!(
            set_is_exception_flag(struct_type),
            MirType::StructRef(MirTypeStructRef {
                is_exception: true,
                ..
            })
        ));
        assert!(matches!(
            set_is_exception_flag(enum_type),
            MirType::EnumRef(MirTypeEnumRef {
                is_exception: true,
                ..
            })
        ));
        assert_eq!(
            set_is_exception_flag(MirType::Primitive(MirTypePrimitive::U8)),
            MirType::Primitive(MirTypePrimitive::U8)
        );
    }
}
