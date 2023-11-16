use crate::codegen::ir::default::IrDefaultValue;
use crate::codegen::ir::field::IrField;

pub(crate) fn generate_field_required_modifier(field: &IrField) -> &str {
    if field.default.is_some() || matches!(field, Optional(_)) {
        ""
    } else {
        "required "
    }
}

pub(crate) fn generate_field_default(
    field: &IrField,
    freezed: bool,
    dart_enums_style: bool,
) -> String {
    field
        .default
        .as_ref()
        .map(|r#default| {
            let r#default = match r#default {
                IrDefaultValue::Str(lit)
                    if !matches!(&field.ty, IrType::Delegate(IrTypeDelegate::String)) =>
                {
                    // Convert the default value to Dart style.
                    if dart_enums_style {
                        IrField::default_value_to_dart_style(lit).into()
                    } else {
                        lit.value().into()
                    }
                }
                _ => default.to_dart(),
            };
            if freezed {
                format!("@Default({default})")
            } else {
                format!("= {default}")
            }
        })
        .unwrap_or_default()
}
