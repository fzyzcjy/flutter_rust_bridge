use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;
use crate::codegen::ir::mir::ty::enumeration::{MirEnumMode, MirVariantKind};
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use itertools::Itertools;

impl WireDartCodecDcoGeneratorDecoderTrait for EnumRefWireDartCodecDcoGenerator<'_> {
    fn generate_impl_decode_body(&self) -> String {
        let enu = self.mir.get(self.context.mir_pack);
        assert_eq!(enu.mode, MirEnumMode::Complex);

        let variants = enu
            .variants()
            .iter()
            .enumerate()
            .map(|(idx, variant)| {
                let args = match &variant.kind {
                    MirVariantKind::Value => "".to_owned(),
                    MirVariantKind::Struct(st) => st
                        .fields
                        .iter()
                        .enumerate()
                        .map(|(idx, field)| {
                            let val =
                                format!("dco_decode_{}(arr[{}]),", field.ty.safe_ident(), idx + 1);
                            if st.is_fields_named {
                                format!("{}: {}", field.name.dart_style(), val)
                            } else {
                                val
                            }
                        })
                        .collect_vec()
                        .join(""),
                };
                format!("case {}: return {}({});", idx, variant.wrapper_name, args)
            })
            .collect_vec();
        format!(
            "final arr = dcoDecodeList(raw);
            switch (dcoDecodePrimitiveInt(arr[0])) {{
                {}
                default: throw Exception(\"unreachable\");
            }}",
            variants.join("\n"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::test_utils;
    use crate::codegen::ir::mir::field::{MirField, MirFieldSettings};
    use crate::codegen::ir::mir::ident::MirIdent;
    use crate::codegen::ir::mir::ty::enumeration::{
        MirEnum, MirEnumIdent, MirEnumMode, MirEnumVariant, MirTypeEnumRef, MirVariantKind,
    };
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

    /// Emits value and named-struct enum cases with their tag-indexed decoders.
    #[test]
    fn enumeration_decoder_covers_value_and_named_struct_variants() {
        let mut pack = test_utils::pack();
        let name = NamespacedName::new(Namespace::default(), "Event".into());
        pack.enum_pool.insert(
            MirEnumIdent(name.clone()),
            MirEnum {
                name: name.clone(),
                wrapper_name: None,
                comments: vec![],
                variants: vec![
                    MirEnumVariant {
                        name: MirIdent::new("empty".into(), None),
                        wrapper_name: MirIdent::new("Event.empty".into(), None),
                        comments: vec![],
                        kind: MirVariantKind::Value,
                    },
                    MirEnumVariant {
                        name: MirIdent::new("value".into(), None),
                        wrapper_name: MirIdent::new("Event.value".into(), None),
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
                    },
                ],
                mode: MirEnumMode::Complex,
                ignore: false,
                needs_json_serializable: false,
            },
        );
        let config = test_utils::config();
        let generator = EnumRefWireDartCodecDcoGenerator::new(
            MirTypeEnumRef {
                ident: MirEnumIdent(name),
                is_exception: false,
            },
            test_utils::context(&pack, &config),
        );
        let output = generator.generate_impl_decode_body();

        assert_eq!(
            output,
            "final arr = dcoDecodeList(raw);\n            switch (dcoDecodePrimitiveInt(arr[0])) {\n                case 0: return Event.empty();\ncase 1: return Event.value(itemValue: dco_decode_i_32(arr[1]),);\n                default: throw Exception(\"unreachable\");\n            }"
        );
    }
}
