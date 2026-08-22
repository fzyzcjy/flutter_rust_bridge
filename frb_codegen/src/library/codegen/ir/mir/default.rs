use std::borrow::Cow;

crate::mir! {
pub enum MirDefaultValue {
    String { content: String },
    Others { dart_literal: String },
}
}

impl MirDefaultValue {
    pub(crate) fn to_dart_literal(&self) -> Cow<'_, str> {
        match self {
            MirDefaultValue::String { content } => format!("r\"{content}\"").into(),
            MirDefaultValue::Others { dart_literal } => dart_literal.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MirDefaultValue;

    /// Renders string and other default values as Dart literals.
    #[test]
    fn renders_string_and_other_literals() {
        assert_eq!(
            MirDefaultValue::String {
                content: "x".into()
            }
            .to_dart_literal(),
            "r\"x\""
        );
        assert_eq!(
            MirDefaultValue::Others {
                dart_literal: "null".into()
            }
            .to_dart_literal(),
            "null"
        );
    }
}
