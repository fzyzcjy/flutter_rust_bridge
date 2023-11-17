use std::borrow::Cow;

crate::ir! {
pub enum IrDefaultValue {
    String { content: String },
    Others { dart_literal: String },
}
}

impl IrDefaultValue {
    pub(crate) fn to_dart_literal(&self) -> Cow<str> {
        match self {
            IrDefaultValue::String { content } => format!("r\"{}\"", content).into(),
            IrDefaultValue::Others { dart_literal } => dart_literal.into(),
        }
    }
}
