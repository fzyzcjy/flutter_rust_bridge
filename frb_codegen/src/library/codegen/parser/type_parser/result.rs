use crate::codegen::ir::result::IrMaybeResult;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{EnumRef, StructRef};
use crate::codegen::parser::type_parser::unencodable::splay_segments;
use crate::codegen::parser::type_parser::{TypeParser, TypeParserParsingContext};

pub(crate) fn parse_type_maybe_result(
    ir: &IrType,
    type_parser: &mut TypeParser,
    context: &TypeParserParsingContext,
) -> anyhow::Result<IrMaybeResult> {
    if let IrType::RustAutoOpaque(inner) = ir {
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

    Ok(IrMaybeResult {
        normal: ir.clone(),
        error: None,
    })
}

fn parse_type_result(args: &[IrType]) -> anyhow::Result<IrMaybeResult> {
    let normal = args.first().unwrap();

    let is_anyhow = args.len() == 1
        || args.iter().any(|x| {
            if let IrType::RustAutoOpaque(inner) = x {
                return inner.raw.string == "anyhow :: Error";
            }
            false
        });

    let error = if is_anyhow {
        Some(IrType::Delegate(IrTypeDelegate::AnyhowException))
    } else {
        args.last().cloned()
    };

    let error = error.map(set_is_exception_flag);

    Ok(IrMaybeResult {
        normal: normal.clone(),
        error,
    })
}

fn set_is_exception_flag(mut ty: IrType) -> IrType {
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
