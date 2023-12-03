use crate::codegen::ir::field::IrField;
use crate::codegen::ir::ty::rust_auto_opaque::IrTypeRustAutoOpaque;
use crate::codegen::ir::ty::IrType;
use crate::codegen::parser::function_parser::FunctionPartialInfo;
use itertools::Itertools;

pub(super) fn transform_fn_info(info: FunctionPartialInfo) -> FunctionPartialInfo {
    FunctionPartialInfo {
        inputs: (info.inputs.into_iter())
            .map(|x| IrField {
                ty: transform_fn_arg_or_output_type(x.ty),
                ..x
            })
            .collect_vec(),
        ok_output: info.ok_output.map(transform_fn_arg_or_output_type),
        error_output: info.error_output.map(transform_fn_arg_or_output_type),
        mode: info.mode,
    }
}

fn transform_fn_arg_or_output_type(ty: IrType) -> IrType {
    if children_type_has_unencodable && !children_type_has_rust_opaque {
        return IrType::RustAutoOpaque(IrTypeRustAutoOpaque::new(TODO, ty));
    }
    ty
}
