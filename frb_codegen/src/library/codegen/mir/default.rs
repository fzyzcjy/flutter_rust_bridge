use std::borrow::Cow;

crate::mir! {
pub enum MirDefaultValue {
    String { content: String },
    Others { dart_literal: String },
}
}

impl MirDefaultValue {
    pub(crate) fn to_dart_literal(&self) -> Cow<str> {
        match self {
            MirDefaultValue::String { content } => format!("r\"{}\"", content).into(),
            MirDefaultValue::Others { dart_literal } => dart_literal.into(),
        }
    }
}
