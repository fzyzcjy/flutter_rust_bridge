use crate::codegen::ir::field::IrField;
use crate::codegen::ir::pack::DistinctTypeGatherer;
use crate::codegen::ir::ty::rust_auto_opaque::IrTypeRustAutoOpaque;
use crate::codegen::ir::ty::{IrContext, IrType};
use crate::codegen::parser::function_parser::{FunctionParser, FunctionPartialInfo};
use itertools::Itertools;

impl<'a, 'b> FunctionParser<'a, 'b> {
    pub(super) fn transform_fn_info(&mut self, info: FunctionPartialInfo) -> FunctionPartialInfo {
        FunctionPartialInfo {
            inputs: (info.inputs.into_iter())
                .map(|x| IrField {
                    ty: self.transform_fn_arg_or_output_type_to_rust_auto_opaque(x.ty),
                    ..x
                })
                .collect_vec(),
            ok_output: (info.ok_output)
                .map(|x| self.transform_fn_arg_or_output_type_to_rust_auto_opaque(x)),
            error_output: (info.error_output)
                .map(|x| self.transform_fn_arg_or_output_type_to_rust_auto_opaque(x)),
            mode: info.mode,
        }
    }

    fn transform_fn_arg_or_output_type_to_rust_auto_opaque(&mut self, ty: IrType) -> IrType {
        self.type_parser.transform_type_rust_auto_opaque(ty)
    }
}
