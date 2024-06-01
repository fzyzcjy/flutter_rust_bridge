use crate::codegen::ir::mir::ty::MirType;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use std::collections::HashMap;

pub(crate) fn generate_class_extra_body(
    mir_type: MirType,
    dart_code_of_type: &HashMap<String, String>,
) -> String {
    dart_code_of_type
        .get(&mir_type.safe_ident())
        .cloned()
        .unwrap_or_default()
}
