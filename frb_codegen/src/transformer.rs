use log::debug;

use crate::ir::IrType::*;
use crate::ir::*;
pub fn transform(src_func: &mut IrFunc) {
    *src_func = IrFunc {
        inputs: src_func
            .inputs
            .iter()
            .map(|f| transform_func_input_add_boxed(f.clone()))
            .collect(),
        ..src_func.clone()
    };
}

fn transform_func_input_add_boxed(input: IrField) -> IrField {
    if input.ty.is_struct_ref_or_enum_ref_or_record() {
        debug!(
            "transform_func_input_add_boxed wrap Boxed to field={:?}",
            input
        );
        IrField {
            ty: tranform_input_type(&input.ty),
            ..input
        }
    } else {
        input
    }
}

pub fn tranform_input_type(input_type: &IrType) -> IrType {
    Boxed(IrTypeBoxed {
        exist_in_real_api: false, // <--
        inner: Box::new(input_type.clone()),
    })
}
