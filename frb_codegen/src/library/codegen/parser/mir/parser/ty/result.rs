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
