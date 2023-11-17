use crate::codegen::ir::default::{IrDefaultValue, IrDefaultValueMode};
use crate::codegen::ir::field::IrField;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::IrType;
use crate::utils::dart_keywords::make_string_keyword_safe;
use convert_case::{Case, Casing};

pub(crate) fn generate_field_required_modifier(field: &IrField) -> &str {
    if field.is_optional() {
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
    if let Some(default_value) = field.default.as_ref() {
        let default_value = match default_value.mode {
            IrDefaultValueMode::String
                if !matches!(&field.ty, IrType::Delegate(IrTypeDelegate::String)) =>
            {
                // Convert the default value to Dart style.
                if dart_enums_style {
                    default_value_to_dart_style(&default_value.literal).into()
                } else {
                    todo
                }
            }
            _ => &default_value.literal,
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

fn default_value_to_dart_style(value: &str) -> String {
    let mut split = value.split('.');
    let enum_name = split.next().unwrap();

    let variant_name = split.next().unwrap().to_string();
    let variant_name = make_string_keyword_safe(variant_name.to_case(Case::Camel));

    format!("{enum_name}.{variant_name}")
}
