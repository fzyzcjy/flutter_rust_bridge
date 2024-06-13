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
    if let Some(default_value) = field.default.as_ref() {
        let default_value = match default_value {
            MirDefaultValue::String { content }
                if !matches!(&field.ty, MirType::Delegate(MirTypeDelegate::String)) =>
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
}
