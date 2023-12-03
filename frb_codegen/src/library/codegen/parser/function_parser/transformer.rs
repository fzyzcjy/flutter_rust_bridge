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
                    ty: self.transform_fn_arg_or_output_type(x.ty),
                    ..x
                })
                .collect_vec(),
            ok_output: (info.ok_output).map(|x| self.transform_fn_arg_or_output_type(x)),
            error_output: (info.error_output).map(|x| self.transform_fn_arg_or_output_type(x)),
            mode: info.mode,
        }
    }

    fn transform_fn_arg_or_output_type(&mut self, ty: IrType) -> IrType {
        let subtree_types_except_rust_opaque = {
            let mut gatherer = DistinctTypeGatherer::new();
            ty.visit_types(
                &mut |ty| {
                    gatherer.add(ty);

                    // skip subtrees inside RustOpaque
                    matches!(ty, IrType::RustOpaque(_))
                },
                self.type_parser,
            );
            gatherer.gather()
        };

        if (subtree_types_except_rust_opaque.iter()).any(|x| matches!(x, IrType::Unencodable(_))) {
            return IrType::RustAutoOpaque(IrTypeRustAutoOpaque::new(todo!(), ty));
        }
        ty
    }
}
