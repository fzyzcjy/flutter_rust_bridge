use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{EnumRef, StructRef};
use crate::codegen::parser::type_parser::unencodable::splay_segments;
use crate::codegen::parser::type_parser::TypeParser;

pub(crate) fn parse_type_maybe_result(
    ir: &IrType,
    type_parser: &mut TypeParser,
) -> anyhow::Result<ResultTypeInfo> {
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

    Ok(ResultTypeInfo {
        ok_output: Some(type_parser.parse_type(ty, context)?),
        error_output: TODO,
    })
}

fn parse_type_result(args: &[IrType]) -> anyhow::Result<ResultTypeInfo> {
    let ok_output = args.first().unwrap();

    let is_anyhow = args.len() == 1
        || args.iter().any(|x| {
            if let IrType::RustAutoOpaque(inner) = x {
                return inner.raw.string == "anyhow :: Error";
            }
            false
        });

    let error_output = if is_anyhow {
        Some(IrType::Delegate(IrTypeDelegate::AnyhowException))
    } else {
        args.last().cloned()
    };

    let error_output = error_output.map(set_is_exception_flag);

    Ok(ResultTypeInfo {
        ok_output: Some(ok_output.clone()),
        error_output,
        ..Default::default()
    })
}

pub(crate) struct ResultTypeInfo {
    pub ok_output: IrType,
    pub error_output: Option<IrType>,
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
