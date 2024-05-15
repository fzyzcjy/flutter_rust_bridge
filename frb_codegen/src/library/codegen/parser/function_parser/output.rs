use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{EnumRef, StructRef};
use crate::codegen::parser::function_parser::{FunctionParser, FunctionPartialInfo};
use crate::codegen::parser::type_parser::result::{
    parse_fn_output_type_result, parse_type_maybe_result,
};
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
        let info = parse_type_maybe_result(&ir, self.type_parser)?;
        Ok(FunctionPartialInfo {
            ok_output: Some(info.ok_output),
            error_output: info.error_output,
            ..Default::default()
        })
    }
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
