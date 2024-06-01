use crate::codegen::mir::ty::MirType;
use crate::library::codegen::mir::ty::MirTypeTrait;
use std::collections::HashMap;

pub(crate) fn generate_class_extra_body(
    ir_type: MirType,
    dart_code_of_type: &HashMap<String, String>,
) -> String {
    dart_code_of_type
        .get(&ir_type.safe_ident())
        .cloned()
        .unwrap_or_default()
}
