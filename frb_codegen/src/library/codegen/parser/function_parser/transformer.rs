use crate::codegen::ir::field::IrField;
use crate::codegen::ir::pack::DistinctTypeGatherer;
use crate::codegen::ir::ty::rust_auto_opaque::IrTypeRustAutoOpaque;
use crate::codegen::ir::ty::{IrContext, IrType};
use crate::codegen::parser::function_parser::FunctionPartialInfo;
use itertools::Itertools;

pub(super) fn transform_fn_info(
    info: FunctionPartialInfo,
    ir_context: &impl IrContext,
) -> FunctionPartialInfo {
    FunctionPartialInfo {
        inputs: (info.inputs.into_iter())
            .map(|x| IrField {
                ty: transform_fn_arg_or_output_type(x.ty, ir_context),
                ..x
            })
            .collect_vec(),
        ok_output: (info.ok_output).map(|x| transform_fn_arg_or_output_type(x, ir_context)),
        error_output: (info.error_output).map(|x| transform_fn_arg_or_output_type(x, ir_context)),
        mode: info.mode,
    }
}

fn transform_fn_arg_or_output_type(ty: IrType, ir_context: &impl IrContext) -> IrType {
    let subtree_types_except_rust_opaque = {
        let mut gatherer = DistinctTypeGatherer::new();
        ty.visit_types(
            &mut |ty| {
                gatherer.add(ty);

                // skip subtrees inside RustOpaque
                matches!(ty, IrType::RustOpaque(_))
            },
            ir_context,
        );
        gatherer.gather()
    };

    if (subtree_types_except_rust_opaque.iter()).any(|x| matches!(x, IrType::Unencodable(_))) {
        return IrType::RustAutoOpaque(IrTypeRustAutoOpaque::new(todo!(), ty));
    }
    ty
}
