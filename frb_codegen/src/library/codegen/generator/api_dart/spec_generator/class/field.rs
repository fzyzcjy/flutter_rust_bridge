use crate::codegen::ir::default::IrDefaultValue;
use crate::codegen::ir::field::IrField;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::IrType;
use crate::utils::dart_keywords::make_string_keyword_safe;
use convert_case::{Case, Casing};
use std::borrow::Cow;

pub(crate) fn generate_field_required_modifier(field: &IrField) -> &str {
    if field.is_optional() {
        ""
    } else {
        "required "
    }
}

// the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
// frb-coverage:ignore-start
pub(crate) fn generate_field_default(
    field: &IrField,
    freezed: bool,
    dart_enums_style: bool,
) -> String {
    // frb-coverage:ignore-end
    if let Some(default_value) = field.default.as_ref() {
        let default_value = match default_value {
            IrDefaultValue::String { content }
                if !matches!(&field.ty, IrType::Delegate(IrTypeDelegate::String)) =>
            {
                default_value_maybe_to_dart_style(content, dart_enums_style)
            }
            _ => default_value.to_dart_literal(),
        };

        if freezed {
            format!("@Default({default_value})")
        } else {
            format!("= {default_value}")
        }
    } else {
        "".to_string()
    }
}

fn default_value_maybe_to_dart_style(value: &str, enable: bool) -> Cow<str> {
    if enable {
        default_value_to_dart_style(value).into()
    } else {
        value.into()
    }
}

fn default_value_to_dart_style(value: &str) -> String {
    let mut split = value.split('.');
    let enum_name = split.next().unwrap();

    let variant_name = split.next().unwrap().to_string();
    let variant_name = make_string_keyword_safe(variant_name.to_case(Case::Camel));

    format!("{enum_name}.{variant_name}")
}
