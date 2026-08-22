use crate::codegen::generator::codec::sse::ty::structure::GeneralizedStructGenerator;
use crate::codegen::generator::codec::sse::ty::*;
use crate::codegen::generator::misc::struct_or_record::StructOrRecord;
use crate::codegen::ir::mir::ty::enumeration::{MirEnum, MirEnumVariant, MirVariantKind};
use crate::library::codegen::generator::codec::sse::lang::LangTrait;
use crate::utils::namespace::NamespacedName;
use itertools::Itertools;

impl CodecSseTyTrait for EnumRefCodecSseTy<'_> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        let src = self.mir.get(self.context.mir_pack);
        Some(generate_enum_encode_rust_general(
            lang,
            src,
            "self",
            |idx, variant| {
                let fields = (variant.kind.fields().iter())
                    .map(|field| {
                        format!(
                            "{};\n",
                            lang.call_encode(&field.ty, &field.name.style(lang, false))
                        )
                    })
                    .join("");

                format!(
                    "{}; {fields}",
                    lang.call_encode(&TAG_TYPE, &format!("{idx}")),
                )
            },
        ))
    }

    fn generate_decode(&self, lang: &Lang) -> Option<String> {
        let src = self.mir.get(self.context.mir_pack);

        let var_decl = lang.var_decl();
        let expr_decode_tag = lang.call_decode(&TAG_TYPE);

        let variants = (src.variants().iter().enumerate())
            .map(|(idx, variant)| {
                (
                    format!("{idx}"),
                    generate_decode_variant(variant, &src.name, lang, self.context),
                )
            })
            .collect_vec();

        let body = lang.switch_expr(
            "tag_",
            &variants,
            Some(format!("{};", lang.throw_unimplemented(""))),
        );

        Some(format!(
            "
            {var_decl} tag_ = {expr_decode_tag};
            {body}
            "
        ))
    }
}

fn generate_decode_variant(
    variant: &MirEnumVariant,
    enum_name: &NamespacedName,
    lang: &Lang,
    context: CodecSseTyContext,
) -> String {
    let enum_name_str = enum_name.style(lang);
    let enum_sep = enum_sep(lang);
    match &variant.kind {
        MirVariantKind::Value => {
            format!(
                "return {enum_name_str}{enum_sep}{}{};",
                variant.name,
                match lang {
                    Lang::DartLang(_) => "()",
                    Lang::RustLang(_) => "",
                }
            )
        }
        MirVariantKind::Struct(st) => {
            GeneralizedStructGenerator::new(st.clone(), context, StructOrRecord::Struct)
                .generate_decode(
                    lang,
                    Some(format!("{enum_name_str}{enum_sep}{}", st.name.name)),
                    false,
                )
        }
    }
}

pub(crate) fn generate_enum_encode_rust_general(
    lang: &Lang,
    src: &MirEnum,
    self_ref: &str,
    generate_branch: impl Fn(usize, &MirEnumVariant) -> String,
) -> String {
    let enum_name_str = src.name.style(lang);
    let enum_sep = enum_sep(lang);
    let variants = (src.variants().iter().enumerate())
        .map(|(idx, variant)| {
            let variant_name = &variant.name;
            let pattern = pattern_match_enum_variant(lang, variant);
            let body = generate_branch(idx, variant);
            (
                format!("{enum_name_str}{enum_sep}{variant_name}{pattern}"),
                body,
            )
        })
        .collect_vec();

    lang.switch_expr(
        self_ref,
        &variants,
        matches!(lang, Lang::RustLang(_)).then(|| format!("{};", lang.throw_unimplemented(""))),
    )
}

fn pattern_match_enum_variant(lang: &Lang, variant: &MirEnumVariant) -> String {
    match &variant.kind {
        MirVariantKind::Value => match lang {
            Lang::DartLang(_) => "()".to_owned(),
            Lang::RustLang(_) => "".to_owned(),
        },
        MirVariantKind::Struct(st) => match lang {
            Lang::DartLang(_) => {
                let pattern = (st.fields.iter())
                    .map(|field| format!("{name}: final {name}", name = field.name.dart_style()))
                    .join(",");
                format!("({pattern})")
            }
            Lang::RustLang(_) => {
                let pattern = (st.fields.iter())
                    .map(|field| field.name.rust_style(false).to_owned())
                    .join(",");
                let (left, right) = st.brackets_pair();
                format!("{left}{pattern}{right}")
            }
        },
    }
}

fn enum_sep(lang: &Lang) -> &'static str {
    match lang {
        Lang::DartLang(_) => "_",
        Lang::RustLang(_) => "::",
    }
}

const TAG_TYPE: MirType = Primitive(MirTypePrimitive::I32);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::codec::sse::lang::{dart::DartLang, rust::RustLang};
    use crate::codegen::ir::mir::field::{MirField, MirFieldSettings};
    use crate::codegen::ir::mir::ident::MirIdent;
    use crate::codegen::ir::mir::ty::structure::MirStruct;
    use crate::utils::namespace::{Namespace, NamespacedName};

    fn value_variant() -> MirEnumVariant {
        MirEnumVariant {
            name: MirIdent::new("Ready".into(), None),
            wrapper_name: MirIdent::new("Ready".into(), None),
            comments: vec![],
            kind: MirVariantKind::Value,
        }
    }

    fn field(name: &str) -> MirField {
        MirField {
            ty: Primitive(MirTypePrimitive::I32),
            name: MirIdent::new(name.into(), None),
            is_final: false,
            is_rust_public: None,
            comments: vec![],
            default: None,
            settings: MirFieldSettings::default(),
        }
    }

    fn struct_variant(named: bool) -> MirEnumVariant {
        MirEnumVariant {
            name: MirIdent::new("Payload".into(), None),
            wrapper_name: MirIdent::new("Payload".into(), None),
            comments: vec![],
            kind: MirVariantKind::Struct(MirStruct {
                name: NamespacedName::new(Namespace::new_raw("crate".into()), "Payload".into()),
                wrapper_name: None,
                fields: vec![field("value")],
                is_fields_named: named,
                dart_metadata_raw: vec![],
                ignore: false,
                needs_json_serializable: false,
                generate_hash: false,
                generate_eq: false,
                dart_collection_deep_equality: false,
                ui_state: false,
                comments: vec![],
            }),
        }
    }

    /// Uses the target-specific enum separator and unit-variant pattern syntax.
    #[test]
    fn value_variant_patterns_and_separators_match_each_target() {
        let variant = value_variant();
        assert_eq!(enum_sep(&Lang::DartLang(DartLang)), "_");
        assert_eq!(
            pattern_match_enum_variant(&Lang::DartLang(DartLang), &variant),
            "()"
        );
        assert_eq!(enum_sep(&Lang::RustLang(RustLang)), "::");
        assert_eq!(
            pattern_match_enum_variant(&Lang::RustLang(RustLang), &variant),
            ""
        );
    }

    /// Destructures named and positional payload variants with target-local patterns.
    #[test]
    fn struct_variant_patterns_cover_dart_and_rust_named_tuple_forms() {
        let dart = Lang::DartLang(DartLang);
        let rust = Lang::RustLang(RustLang);
        assert_eq!(
            pattern_match_enum_variant(&dart, &struct_variant(true)),
            "(value: final value)"
        );
        assert_eq!(
            pattern_match_enum_variant(&rust, &struct_variant(true)),
            "{value}"
        );
        assert_eq!(
            pattern_match_enum_variant(&rust, &struct_variant(false)),
            "(value)"
        );
    }
}
