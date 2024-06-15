use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::misc::dart_wire_type_from_rust_wire_type_or_web;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;
use crate::codegen::ir::mir::ty::enumeration::{MirEnumVariant, MirVariantKind};
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use itertools::Itertools;

impl<'a> WireDartCodecCstGeneratorEncoderTrait for EnumRefWireDartCodecCstGenerator<'a> {
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

impl<'a> EnumRefWireDartCodecCstGenerator<'a> {
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
                            field.name.rust_style(),
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
                        format!("{r}.{name} = pre_{name};", name = field.name.rust_style())
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
        variant = variant.wrapper_name.rust_style(),
    )
}
