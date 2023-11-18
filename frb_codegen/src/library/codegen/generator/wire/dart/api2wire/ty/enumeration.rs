use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::dart::api2wire::ty::WireDartGeneratorApi2wireTrait;
use crate::codegen::generator::wire::dart::base::*;
use crate::codegen::ir::ty::enumeration::{IrVariant, IrVariantKind};
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;

impl<'a> WireDartGeneratorApi2wireTrait for EnumRefWireDartGenerator<'a> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        let variants = (self.ir.get(self.context.ir_pack).variants())
            .iter()
            .enumerate()
            .map(|(idx, variant)| generate_api2wire_body_variant(idx, &variant))
            .join("\n");

        Acc {
            wasm: Some(format!(
                "{variants}

                throw Exception('unreachable');"
            )),
            ..Default::default()
        }
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        Some(
            self.ir
                .get(self.context.ir_pack)
                .variants()
                .iter()
                .enumerate()
                .map(|(idx, variant)| self.generate_api_fill_to_wire_body_variant(idx, variant))
                .join("\n"),
        )
    }
}

impl<'a> EnumRefWireDartGenerator<'a> {
    fn generate_api_fill_to_wire_body_variant(&self, index: usize, variant: &IrVariant) -> String {
        let wrapper_name = &variant.wrapper_name.raw;
        let variant_name = &variant.name.raw;

        match &variant.kind {
            IrVariantKind::Value => {
                format!("if (apiObj is {wrapper_name}) {{ wireObj.tag = {index}; return; }}",)
            }
            IrVariantKind::Struct(st) => {
                let pre_field = st
                    .fields
                    .iter()
                    .map(|field| {
                        format!(
                            "var pre_{} = api2wire_{}(apiObj.{});",
                            field.name.rust_style(),
                            field.ty.safe_ident(),
                            field.name.dart_style()
                        )
                    })
                    .join("\n");

                let r = format!("wireObj.kind.ref.{variant_name}.ref");

                let body = st
                    .fields
                    .iter()
                    .map(|field| {
                        format!("{r}.{name} = pre_{name};", name = field.name.rust_style(),)
                    })
                    .join("\n");

                format!(
                    "if (apiObj is {wrapper_name}) {{
                        {pre_field}
                        wireObj.tag = {index};
                        wireObj.kind = inner.inflate_{ident}_{variant_name}();
                        {body}
                        return;
                    }}",
                    ident = self.ir.ident.0,
                )
            }
        }
    }
}

fn generate_api2wire_body_variant(idx: usize, variant: &&IrVariant) -> String {
    let fields = match &variant.kind {
        IrVariantKind::Value => vec![],
        IrVariantKind::Struct(st) => (st.fields)
            .iter()
            .map(|field| {
                format!(
                    ",api2wire_{}(raw.{})",
                    field.ty.safe_ident(),
                    field.name.dart_style()
                )
            })
            .collect(),
    }
    .join("");
    format!(
        "if (raw is {variant}) {{
                        return [{} {}];
                    }}",
        idx,
        fields,
        variant = variant.wrapper_name.rust_style(),
    )
}
