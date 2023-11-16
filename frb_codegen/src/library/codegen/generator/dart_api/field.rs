use crate::codegen::ir::field::IrField;

pub(crate) fn generate_field_required_modifier(field: &IrField) -> &str {
    if field.default.is_some() || matches!(field, Optional(_)) {
        ""
    } else {
        "required "
    }
}
