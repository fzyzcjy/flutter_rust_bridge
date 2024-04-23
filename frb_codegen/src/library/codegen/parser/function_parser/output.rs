use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{EnumRef, StructRef};
use crate::codegen::parser::function_parser::{FunctionParser, FunctionPartialInfo};
use crate::codegen::parser::type_parser::unencodable::splay_segments;
use crate::codegen::parser::type_parser::TypeParserParsingContext;
use syn::*;

impl<'a, 'b> FunctionParser<'a, 'b> {
    pub(super) fn parse_fn_output(
        &mut self,
        sig: &Signature,
        context: &TypeParserParsingContext,
    ) -> anyhow::Result<FunctionPartialInfo> {
        Ok(match &sig.output {
            ReturnType::Type(_, ty) => {
                remove_primitive_unit(self.parse_fn_output_type(ty, context)?)
            }
            ReturnType::Default => Default::default(),
        })
    }

    #[allow(clippy::single_match)] // deliberate do so to ensure style consistency
    fn parse_fn_output_type(
        &mut self,
        ty: &Type,
        context: &TypeParserParsingContext,
    ) -> anyhow::Result<FunctionPartialInfo> {
        let ir = self.type_parser.parse_type(ty, context)?;

        if let IrType::RustAutoOpaque(inner) = ir {
            match splay_segments(&inner.raw.segments).last() {
                Some(("Result", args)) => {
                    return parse_fn_output_type_result(
                        &(args.iter())
                            .map(|arg| self.type_parser.parse_type(arg, context))
                            .collect::<anyhow::Result<Vec<_>>>()?,
                    );
                }
                _ => {}
            }
        }

        Ok(FunctionPartialInfo {
            ok_output: Some(self.type_parser.parse_type(ty, context)?),
            ..Default::default()
        })
    }
}

fn parse_fn_output_type_result(args: &[IrType]) -> anyhow::Result<FunctionPartialInfo> {
    let ok_output = args.first();

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

    Ok(FunctionPartialInfo {
        ok_output: ok_output.map(|o| o.clone()),
        error_output,
        ..Default::default()
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

// Convert primitive Unit type -> None
fn remove_primitive_unit(info: FunctionPartialInfo) -> FunctionPartialInfo {
    let ok_output = if info.ok_output == Some(IrType::Primitive(IrTypePrimitive::Unit)) {
        None
    } else {
        info.ok_output
    };

    FunctionPartialInfo { ok_output, ..info }
}
