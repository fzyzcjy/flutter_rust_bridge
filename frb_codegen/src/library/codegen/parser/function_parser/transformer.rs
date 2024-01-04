use crate::codegen::ir::field::IrField;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::primitive_list::IrTypePrimitiveList;
use crate::codegen::ir::ty::IrType;
use crate::codegen::parser::function_parser::{FunctionParser, FunctionPartialInfo};
use crate::codegen::parser::type_parser::TypeParserParsingContext;
use itertools::Itertools;
use IrTypePrimitive::{Isize, Usize, I64, U64};

impl<'a, 'b> FunctionParser<'a, 'b> {
    pub(super) fn transform_fn_info(
        &mut self,
        info: FunctionPartialInfo,
        context: &TypeParserParsingContext,
    ) -> FunctionPartialInfo {
        FunctionPartialInfo {
            inputs: (info.inputs.into_iter())
                .map(|x| IrField {
                    ty: transform_primitive_list_param(
                        self.transform_fn_arg_or_output_type_to_rust_auto_opaque(x.ty, context),
                    ),
                    ..x
                })
                .collect_vec(),
            ok_output: (info.ok_output)
                .map(|x| self.transform_fn_arg_or_output_type_to_rust_auto_opaque(x, context)),
            error_output: (info.error_output)
                .map(|x| self.transform_fn_arg_or_output_type_to_rust_auto_opaque(x, context)),
            mode: info.mode,
        }
    }

    fn transform_fn_arg_or_output_type_to_rust_auto_opaque(
        &mut self,
        ty: IrType,
        context: &TypeParserParsingContext,
    ) -> IrType {
        self.type_parser
            .transform_type_rust_auto_opaque(&ty, context)
    }
}

fn transform_primitive_list_param(ty: IrType) -> IrType {
    if let IrType::PrimitiveList(inner) = &ty {
        match inner.primitive {
            U64 | I64 | Usize | Isize => ty,
            _ => IrType::PrimitiveList(IrTypePrimitiveList {
                strict_dart_type: false,
                ..inner.clone()
            }),
        }
    } else {
        ty
    }
}
