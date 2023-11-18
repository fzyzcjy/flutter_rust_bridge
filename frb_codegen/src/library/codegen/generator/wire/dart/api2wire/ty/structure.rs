use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::dart::api2wire::ty::WireDartGeneratorApi2wireTrait;
use crate::codegen::generator::wire::dart::base::*;
use crate::codegen::ir::ty::structure::IrTypeStructRef;
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;
use GeneralizedStructGeneratorMode::Struct;

impl<'a> WireDartGeneratorApi2wireTrait for StructRefWireDartGenerator<'a> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        self.new_generalized_generator().api2wire_body()
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        self.new_generalized_generator().api_fill_to_wire_body()
    }
}

impl<'a> StructRefWireDartGenerator<'a> {
    fn new_generalized_generator(&self) -> GeneralizedStructGenerator {
        GeneralizedStructGenerator::new(self.ir.clone(), self.context, Struct)
    }
}

pub(super) enum GeneralizedStructGeneratorMode {
    Struct,
    Record,
}

pub(super) struct GeneralizedStructGenerator<'a> {
    ir: IrTypeStructRef,
    context: WireDartGeneratorContext<'a>,
    mode: GeneralizedStructGeneratorMode,
}

impl<'a> GeneralizedStructGenerator<'a> {
    pub fn new(
        ir: IrTypeStructRef,
        context: WireDartGeneratorContext<'a>,
        mode: GeneralizedStructGeneratorMode,
    ) -> Self {
        Self { ir, context, mode }
    }

    fn api2wire_body(&self) -> Acc<Option<String>> {
        Acc {
            wasm: self.context.config.wasm_enabled.then(|| {
                format!(
                    "return [{}];",
                    self.ir
                        .get(self.context.ir_pack)
                        .fields
                        .iter()
                        .map(|field| {
                            format!(
                                "api2wire_{}(raw.{})",
                                field.ty.safe_ident(),
                                field.name.dart_style()
                            )
                        })
                        .collect_vec()
                        .join(",")
                )
            }),
            ..Default::default()
        }
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        let s = self.ir.get(self.context.ir_pack);
        Some(
            s.fields
                .iter()
                .map(|field| {
                    self.generate_api_fill_to_wire_body_struct_field(
                        &field.ty.safe_ident(),
                        &field.name.dart_style(),
                        field.name.rust_style(),
                        field.ty.is_struct(),
                    )
                })
                .collect_vec()
                .join("\n"),
        )
    }

    fn generate_api_fill_to_wire_body_struct_field(
        safe_ident: &str,
        dart_style: &str,
        rust_style: &str,
        is_struct: bool,
    ) -> String {
        if is_struct {
            format!(
                "_api_fill_to_wire_{}(apiObj.{}, wireObj.{});",
                safe_ident, dart_style, rust_style
            )
        } else {
            format!(
                "wireObj.{} = api2wire_{}(apiObj.{});",
                rust_style, safe_ident, dart_style
            )
        }
    }
}
