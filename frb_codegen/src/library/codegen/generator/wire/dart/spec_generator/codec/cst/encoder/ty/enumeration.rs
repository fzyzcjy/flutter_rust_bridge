use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::misc::dart_wire_type_from_rust_wire_type_or_web;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;
use crate::codegen::ir::mir::ty::enumeration::{MirEnumVariant, MirVariantKind};
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use itertools::Itertools;

impl WireDartCodecCstGeneratorEncoderTrait for EnumRefWireDartCodecCstGenerator<'_> {
    fn generate_encode_func_body(&self) -> Acc<Option<String>> {
        let variants = (self.mir.get(self.context.mir_pack).variants())
            .iter()
            .enumerate()
            .map(|(idx, variant)| generate_encode_body_variant(idx, variant))
            .join("\n");

        Acc {
            web: Some(format!(
                "{variants}

                throw Exception('unreachable');"
            )),
            ..Default::default()
        }
    }

    fn generate_encode_api_fill_to_wire_body(&self) -> Option<String> {
        Some(
            self.mir
                .get(self.context.mir_pack)
                .variants()
                .iter()
                .enumerate()
                .map(|(idx, variant)| self.generate_api_fill_to_wire_body_variant(idx, variant))
                .join("\n"),
        )
    }

    fn dart_wire_type(&self, target: Target) -> String {
        dart_wire_type_from_rust_wire_type_or_web(self, target, "JSAny".into())
    }
}

impl EnumRefWireDartCodecCstGenerator<'_> {
    fn generate_api_fill_to_wire_body_variant(
        &self,
        index: usize,
        variant: &MirEnumVariant,
    ) -> String {
        let wrapper_name = &variant.wrapper_name;
        let variant_name = &variant.name;

        let (stmt_prepare, stmt_postpare) = match &variant.kind {
            MirVariantKind::Value => ("".to_owned(), "".to_owned()),
            MirVariantKind::Struct(st) => {
                let pre_field = st
                    .fields
                    .iter()
                    .map(|field| {
                        format!(
                            "var pre_{} = cst_encode_{}(apiObj.{});",
                            field.name.rust_style(true),
                            field.ty.safe_ident(),
                            field.name.dart_style()
                        )
                    })
                    .join("\n");

                let r = format!("wireObj.kind.{variant_name}");
                let body = st
                    .fields
                    .iter()
                    .map(|field| {
                        format!(
                            "{r}.{name} = pre_{name};",
                            name = field.name.rust_style(true)
                        )
                    })
                    .join("\n");

                (pre_field, body)
            }
        };

        format!(
            "if (apiObj is {wrapper_name}) {{
                {stmt_prepare}wireObj.tag = {index};{stmt_postpare}
                return;
            }}",
        )
    }
}

fn generate_encode_body_variant(index: usize, variant: &MirEnumVariant) -> String {
    let fields = match &variant.kind {
        MirVariantKind::Value => vec![],
        MirVariantKind::Struct(st) => (st.fields)
            .iter()
            .map(|field| {
                format!(
                    ",cst_encode_{}(raw.{})",
                    field.ty.safe_ident(),
                    field.name.dart_style()
                )
            })
            .collect(),
    }
    .join("");
    format!(
        "if (raw is {variant}) {{
            return [{index} {fields}].jsify()!;
        }}",
        variant = variant.wrapper_name.rust_style(true),
    )
}

#[cfg(test)]
mod tests {
    use super::super::test_utils;
    use super::*;
    use crate::codegen::ir::mir::field::{MirField, MirFieldSettings};
    use crate::codegen::ir::mir::ident::MirIdent;
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use crate::codegen::ir::mir::ty::structure::MirStruct;
    use crate::codegen::ir::mir::ty::MirType;
    use crate::utils::namespace::{Namespace, NamespacedName};

    fn field(name: &str) -> MirField {
        MirField {
            ty: MirType::Primitive(MirTypePrimitive::I32),
            name: MirIdent::new(name.into(), None),
            is_final: true,
            is_rust_public: None,
            comments: vec![],
            default: None,
            settings: MirFieldSettings::default(),
        }
    }

    /// Emits tagged JavaScript arrays for both value and field-bearing enum variants.
    #[test]
    fn enum_encoder_emits_value_and_struct_variant_branches() {
        let name = NamespacedName::new(Namespace::default(), "Event".into());
        let value = MirEnumVariant {
            name: MirIdent::new("empty".into(), None),
            wrapper_name: MirIdent::new("EventEmpty".into(), None),
            comments: vec![],
            kind: MirVariantKind::Value,
        };
        let structured = MirEnumVariant {
            name: MirIdent::new("value".into(), None),
            wrapper_name: MirIdent::new("EventValue".into(), None),
            comments: vec![],
            kind: MirVariantKind::Struct(MirStruct {
                name,
                wrapper_name: None,
                fields: vec![field("item_value")],
                is_fields_named: true,
                dart_metadata_raw: vec![],
                ignore: false,
                needs_json_serializable: false,
                generate_hash: false,
                generate_eq: false,
                dart_collection_deep_equality: false,
                ui_state: false,
                comments: vec![],
            }),
        };

        assert_eq!(
            generate_encode_body_variant(0, &value),
            "if (raw is EventEmpty) {\n            return [0 ].jsify()!;\n        }"
        );
        assert_eq!(generate_encode_body_variant(1, &structured), "if (raw is EventValue) {\n            return [1 ,cst_encode_i_32(raw.itemValue)].jsify()!;\n        }");
    }

    /// Generates web encoding and IO field filling from a pool-backed enum fixture.
    #[test]
    fn enum_encoder_generates_web_and_io_bodies_from_mir_context() {
        let name = NamespacedName::new(Namespace::default(), "Event".into());
        let value = MirEnumVariant {
            name: MirIdent::new("empty".into(), None),
            wrapper_name: MirIdent::new("EventEmpty".into(), None),
            comments: vec![],
            kind: MirVariantKind::Value,
        };
        let structured = MirEnumVariant {
            name: MirIdent::new("value".into(), None),
            wrapper_name: MirIdent::new("EventValue".into(), None),
            comments: vec![],
            kind: MirVariantKind::Struct(MirStruct {
                name: name.clone(),
                wrapper_name: None,
                fields: vec![field("item_value")],
                is_fields_named: true,
                dart_metadata_raw: vec![],
                ignore: false,
                needs_json_serializable: false,
                generate_hash: false,
                generate_eq: false,
                dart_collection_deep_equality: false,
                ui_state: false,
                comments: vec![],
            }),
        };
        let mut pack = test_utils::pack();
        pack.enum_pool.insert(
            crate::codegen::ir::mir::ty::enumeration::MirEnumIdent(name.clone()),
            crate::codegen::ir::mir::ty::enumeration::MirEnum {
                name: name.clone(),
                wrapper_name: None,
                comments: vec![],
                variants: vec![value, structured],
                mode: crate::codegen::ir::mir::ty::enumeration::MirEnumMode::Complex,
                ignore: false,
                needs_json_serializable: false,
            },
        );
        let api_dart_config = test_utils::api_dart_config();
        let wire_dart_config = test_utils::wire_dart_config(true);
        let wire_rust_config = test_utils::wire_rust_config(true);
        let generator = EnumRefWireDartCodecCstGenerator::new(
            crate::codegen::ir::mir::ty::enumeration::MirTypeEnumRef {
                ident: crate::codegen::ir::mir::ty::enumeration::MirEnumIdent(name),
                is_exception: false,
            },
            test_utils::context(
                &pack,
                &wire_dart_config,
                &wire_rust_config,
                &api_dart_config,
            ),
        );

        let encoded = generator.generate_encode_func_body();
        assert_eq!(
            encoded.web.as_deref(),
            Some("if (raw is EventEmpty) {\n            return [0 ].jsify()!;\n        }\nif (raw is EventValue) {\n            return [1 ,cst_encode_i_32(raw.itemValue)].jsify()!;\n        }\n\n                throw Exception('unreachable');")
        );
        assert_eq!(
            generator.generate_encode_api_fill_to_wire_body().as_deref(),
            Some("if (apiObj is EventEmpty) {\n                wireObj.tag = 0;\n                return;\n            }\nif (apiObj is EventValue) {\n                var pre_item_value = cst_encode_i_32(apiObj.itemValue);wireObj.tag = 1;wireObj.kind.value.item_value = pre_item_value;\n                return;\n            }")
        );
    }
}
