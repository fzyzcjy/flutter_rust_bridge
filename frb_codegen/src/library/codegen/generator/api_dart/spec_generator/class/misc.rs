use crate::codegen::ir::ty::IrType;
use crate::library::codegen::ir::ty::IrTypeTrait;
use std::collections::HashMap;

pub(crate) fn generate_class_extra_body(
    ir_type: IrType,
    dart_code_of_type: &HashMap<String, String>,
) -> String {
    dart_code_of_type
        .get(&ir_type.safe_ident())
        .cloned()
        .unwrap_or_default()
}
