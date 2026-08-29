use crate::codegen::ir::mir::comment::MirComment;
use crate::codegen::ir::mir::default::MirDefaultValue;
use crate::codegen::ir::mir::ident::MirIdent;
use crate::codegen::ir::mir::ty::MirType;
use serde::Deserialize;

crate::mir! {
pub struct MirField {
    pub ty: MirType,
    pub name: MirIdent,
    pub is_final: bool,
    pub is_rust_public: Option<bool>,
    pub comments: Vec<MirComment>,
    pub default: Option<MirDefaultValue>,
    pub settings: MirFieldSettings,
}

#[derive(Deserialize, Default)]
pub struct MirFieldSettings {
    pub is_in_mirrored_enum: bool,
    pub skip_auto_accessors: bool,
}
}

impl MirField {
    #[inline]
    pub fn is_optional(&self) -> bool {
        matches!(&self.ty, MirType::Optional(_)) || self.default.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::{MirField, MirFieldSettings};
    use crate::codegen::ir::mir::default::MirDefaultValue;
    use crate::codegen::ir::mir::ident::MirIdent;
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use crate::codegen::ir::mir::ty::MirType;

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

    /// Recognizes explicit optional types and fields with defaults.
    #[test]
    fn optionality_covers_optional_type_and_default() {
        assert!(field(
            MirType::Optional(crate::codegen::ir::mir::ty::optional::MirTypeOptional::new(
                MirType::Primitive(MirTypePrimitive::U8),
            )),
            None,
        )
        .is_optional());
        assert!(!field(MirType::Primitive(MirTypePrimitive::U8), None).is_optional());
        assert!(field(
            MirType::Primitive(MirTypePrimitive::U8),
            Some(MirDefaultValue::Others {
                dart_literal: "0".into()
            })
        )
        .is_optional());
    }
}
