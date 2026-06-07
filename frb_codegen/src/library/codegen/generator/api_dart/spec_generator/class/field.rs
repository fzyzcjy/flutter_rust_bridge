use crate::codegen::ir::mir::default::MirDefaultValue;
use crate::codegen::ir::mir::field::MirField;
use crate::codegen::ir::mir::ty::delegate::MirTypeDelegate;
use crate::codegen::ir::mir::ty::MirType;
use crate::utils::dart_keywords;
use convert_case::{Case, Casing};
use std::borrow::Cow;

pub(crate) fn generate_field_required_modifier(field: &MirField) -> &str {
    if field.is_optional() {
        ""
    } else {
        "required "
    }
}

// the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
// frb-coverage:ignore-start
pub(crate) fn generate_field_default(
    field: &MirField,
    freezed: bool,
    dart_enums_style: bool,
) -> String {
    // frb-coverage:ignore-end
    let Some(default_value) = compute_default_value(field, dart_enums_style) else {
        return "".to_string();
    };

    if freezed {
        format!("@Default({default_value})")
    } else {
        format!("= {default_value}")
    }
}

pub(crate) fn generate_field_default_for_constructor(
    field: &MirField,
    dart_enums_style: bool,
) -> String {
    compute_default_value(field, dart_enums_style)
        .map(|default_value| format!("= {}", ensure_const_default_value(default_value)))
        .unwrap_or_default()
}

fn compute_default_value(field: &MirField, dart_enums_style: bool) -> Option<String> {
    if let Some(default_value) = field.default.as_ref() {
        let default_value = match default_value {
            MirDefaultValue::String { content }
                if !matches!(&field.ty, MirType::Delegate(MirTypeDelegate::String)) =>
            {
                default_value_maybe_to_dart_style(content, dart_enums_style)
            }
            _ => default_value.to_dart_literal(),
        };

        Some(default_value.to_string())
    } else {
        None
    }
}

fn ensure_const_default_value(default_value: String) -> String {
    if default_value.contains('(')
        && !default_value.starts_with("const ")
        && !is_dart_string_literal(&default_value)
        && !is_parenthesized_expression(&default_value)
    {
        format!("const {default_value}")
    } else {
        default_value
    }
}

fn is_parenthesized_expression(value: &str) -> bool {
    value.starts_with('(')
}

fn is_dart_string_literal(value: &str) -> bool {
    value.starts_with('\'')
        || value.starts_with('"')
        || value.starts_with("r'")
        || value.starts_with("r\"")
}

fn default_value_maybe_to_dart_style(value: &str, enable: bool) -> Cow<'_, str> {
    if enable {
        default_value_to_dart_style(value).into()
    } else {
        value.into()
    }
}

fn default_value_to_dart_style(value: &str) -> String {
    match value.split_once('.') {
        // If the user is explicitly calling an enum variant's constructor
        // i.e. `const Foo.bar()` instead of `Foo.Bar`, we trust that they
        // really mean it and don't convert.
        Some((enum_name, variant_name))
            if !enum_name.starts_with("const ") && !variant_name.contains('(') =>
        {
            format!(
                "{}.{}",
                enum_name,
                dart_keywords::escape(variant_name.to_case(Case::Camel))
            )
        }
        _ => value.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_default_value_to_dart_style() {
        assert_eq!(&default_value_to_dart_style("something"), "something");
        assert_eq!(
            &default_value_to_dart_style("OneTwo.ThreeFour"),
            "OneTwo.threeFour"
        );
        assert_eq!(
            &default_value_to_dart_style("const Foo.bar()"),
            "const Foo.bar()"
        );
    }

    #[test]
    pub fn test_ensure_const_default_value() {
        assert_eq!(
            &ensure_const_default_value("Foo.bar()".to_string()),
            "const Foo.bar()"
        );
        assert_eq!(
            &ensure_const_default_value("const Foo.bar()".to_string()),
            "const Foo.bar()"
        );
        assert_eq!(
            &ensure_const_default_value("Foo.bar".to_string()),
            "Foo.bar"
        );
        assert_eq!(
            &ensure_const_default_value("'hello()'".to_string()),
            "'hello()'"
        );
        assert_eq!(
            &ensure_const_default_value("r'hello()'".to_string()),
            "r'hello()'"
        );
        assert_eq!(
            &ensure_const_default_value("(1 + 2)".to_string()),
            "(1 + 2)"
        );
    }
}
