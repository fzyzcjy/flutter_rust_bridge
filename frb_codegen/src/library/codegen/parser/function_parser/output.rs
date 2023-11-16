use crate::codegen::ir::func::IrFuncMode;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::unencodable::IrTypeUnencodable;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{EnumRef, StructRef};
use crate::codegen::parser::function_parser::{
    type_to_string, FunctionParser, FunctionPartialInfo,
};
use crate::codegen::parser::type_parser::unencodable::{splay_segments, ArgsRefs};
use crate::codegen::parser::ParserResult;
use anyhow::Context;
use syn::*;

impl<'a, 'b> FunctionParser<'a, 'b> {
    pub(super) fn parse_fn_output(&mut self, sig: &Signature) -> ParserResult<FunctionPartialInfo> {
        Ok(match &sig.output {
            ReturnType::Type(_, ty) => self.parse_fn_output_type(ty)?,
            ReturnType::Default => Default::default(),
        })
    }

    fn parse_fn_output_type(&mut self, ty: &Type) -> ParserResult<FunctionPartialInfo> {
        let ir = self.type_parser.parse_type(ty)?;

        if let IrType::Unencodable(IrTypeUnencodable { segments, .. }) = ir {
            match splay_segments(&segments).last() {
                Some(("Result", Some(ArgsRefs::Generic(args)))) => {
                    return parse_fn_output_type_result(args);
                }
                _ => {}
            }
        }

        Ok(FunctionPartialInfo {
            ok_output: Some(self.type_parser.parse_type(ty)?),
            ..Default::default()
        })
    }
}

fn parse_fn_output_type_result(args: &[IrType]) -> ParserResult<FunctionPartialInfo> {
    let ok_output = args.first().unwrap();

    let is_anyhow = args.len() == 1
        || args.iter().any(|x| match x {
            IrType::Unencodable(IrTypeUnencodable { string, .. }) => string == "anyhow :: Error",
            _ => false,
        });

    let error_output = if is_anyhow {
        Some(IrType::Delegate(IrTypeDelegate::Anyhow))
    } else {
        args.last().cloned()
    };

    let error_output = error_output.map(set_is_exception_flag);

    Ok(FunctionPartialInfo {
        ok_output: Some(ok_output.clone()),
        error_output,
        ..Default::default()
    })
}

fn set_is_exception_flag(mut ty: IrType) -> IrType {
    match &mut ty {
        StructRef(mut inner) => {
            inner.is_exception = true;
        }
        EnumRef(mut inner) => {
            inner.is_exception = true;
        }
        _ => {}
    }
    ty
}
