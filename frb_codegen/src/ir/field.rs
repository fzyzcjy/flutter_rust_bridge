use convert_case::Casing;

use crate::{ir::*, parser::DefaultValues};

#[derive(Debug, Clone)]
pub struct IrField {
    pub ty: IrType,
    pub name: IrIdent,
    pub is_final: bool,
    pub comments: Vec<IrComment>,
    pub default: Option<DefaultValues>,
}

impl IrField {
    #[inline]
    pub fn required_modifier(&self) -> &str {
        if self.default.is_some() {
            ""
        } else {
            self.ty.dart_required_modifier()
        }
    }
    pub fn field_default(&self, freezed: bool) -> String {
        self.default
            .as_ref()
            .map(|r#default| {
                let r#default = match r#default {
                    DefaultValues::Str(lit)
                        if !matches!(&self.ty, IrType::Delegate(IrTypeDelegate::String)) =>
                    {
                        if cfg!(feature = "dart-style-enums") {
                            let value = lit.value();
                            let mut split = value.split('.');
                            let enum_name = split.next().unwrap();
                            let variant_name = split.next().unwrap().to_case(convert_case::Case::Camel);
                            format!("{enum_name}.{variant_name}").into()
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

    #[inline]
    pub fn is_optional(&self) -> bool {
        matches!(&self.ty, IrType::Optional(_)) || self.default.is_some()
    }
}
