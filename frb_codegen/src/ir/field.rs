use convert_case::{Casing, Case};

use crate::{ir::*, parser::DefaultValues, Opts};

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
    pub fn field_default(&self, freezed: bool, config: Option<&Opts>) -> String {
        self.default
            .as_ref()
            .map(|r#default| {
                let r#default = match r#default {
                    DefaultValues::Str(lit)
                        if !matches!(&self.ty, IrType::Delegate(IrTypeDelegate::String)) =>
                    {
                        // Convert the default value to Dart style.
                        if config.is_some() && config.unwrap().dart_enums_style {
                            let value = lit.value();
                            let mut split = value.split('.');
                            let enum_name = split.next().unwrap();

                            let variant_name = split.next().unwrap().to_string();
                            let variant_name = if crate::utils::check_for_keywords(&[variant_name.clone()]).is_err() {
                                variant_name.to_case(Case::Pascal)
                            } else {
                                variant_name.to_case(Case::Camel)
                            };

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
