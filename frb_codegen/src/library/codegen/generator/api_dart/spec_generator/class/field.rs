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
    use crate::codegen::ir::mir::field::MirFieldSettings;
    use crate::codegen::ir::mir::ident::MirIdent;
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;

    fn field(ty: MirType, default: Option<MirDefaultValue>) -> MirField {
        MirField {
            ty,
            name: MirIdent::new("value".into(), None),
            is_final: false,
            is_rust_public: None,
            comments: vec![],
            default,
            settings: MirFieldSettings::default(),
        }
    }

    #[test]
    /// Marks fields without defaults as required and defaulted fields as optional.
    fn generates_required_modifier_from_field_optionality() {
        assert_eq!(
            generate_field_required_modifier(&field(
                MirType::Primitive(MirTypePrimitive::I32),
                None
            )),
            "required "
        );
        assert_eq!(
            generate_field_required_modifier(&field(
                MirType::Primitive(MirTypePrimitive::I32),
                Some(MirDefaultValue::Others {
                    dart_literal: "0".into(),
                }),
            )),
            ""
        );
    }

    #[test]
    /// Renders absent, string, and non-string defaults for Freezed and plain fields.
    fn generates_default_annotations_and_assignments() {
        assert_eq!(
            generate_field_default(
                &field(MirType::Primitive(MirTypePrimitive::I32), None),
                false,
                false
            ),
            ""
        );
        assert_eq!(
            generate_field_default(
                &field(
                    MirType::Primitive(MirTypePrimitive::I32),
                    Some(MirDefaultValue::String {
                        content: "Example.Class".into(),
                    }),
                ),
                false,
                false,
            ),
            "= Example.Class"
        );
        assert_eq!(
            generate_field_default(
                &field(
                    MirType::Delegate(MirTypeDelegate::String),
                    Some(MirDefaultValue::String {
                        content: "value".into(),
                    }),
                ),
                true,
                true,
            ),
            "@Default(r\"value\")"
        );
        assert_eq!(
            generate_field_default(
                &field(
                    MirType::Primitive(MirTypePrimitive::I32),
                    Some(MirDefaultValue::Others {
                        dart_literal: "42".into(),
                    }),
                ),
                false,
                true,
            ),
            "= 42"
        );
    }

    #[test]
    /// Converts eligible enum defaults only when Dart enum style is enabled.
    fn applies_dart_enum_style_only_to_non_string_defaults() {
        let enum_default = || {
            field(
                MirType::Primitive(MirTypePrimitive::I32),
                Some(MirDefaultValue::String {
                    content: "Example.Class".into(),
                }),
            )
        };

        assert_eq!(
            generate_field_default(&enum_default(), false, true),
            "= Example.class_"
        );
        assert_eq!(
            generate_field_default(&enum_default(), false, false),
            "= Example.Class"
        );
    }

    #[test]
    /// Converts enum defaults to Dart camel case while preserving constructor expressions.
    fn converts_enum_defaults_to_dart_style() {
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
    /// Escapes Dart keywords after converting enum variant names to camel case.
    fn escapes_dart_keywords_in_converted_enum_defaults() {
        assert_eq!(
            default_value_to_dart_style("Example.Class"),
            "Example.class_"
        );
    }
}
