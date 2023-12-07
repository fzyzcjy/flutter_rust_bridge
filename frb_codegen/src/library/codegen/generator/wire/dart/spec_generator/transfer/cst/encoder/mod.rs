use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::{Target, TargetOrCommon};
use crate::codegen::generator::wire::dart::spec_generator::base::{
    WireDartGenerator, WireDartGeneratorContext,
};
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::base::{
    WireDartTransferCstGenerator, WireDartTransferCstGeneratorContext,
};
use crate::codegen::ir::pack::IrPackComputedCache;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::Optional;
use crate::library::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;
use serde::Serialize;

mod misc;
pub(crate) mod ty;

#[derive(Clone, Serialize)]
pub(crate) struct WireDartOutputSpecTransferCstEncoder {
    pub(crate) api2wire_funcs: Acc<Vec<WireDartOutputCode>>,
    pub(crate) api_fill_to_wire_funcs: Acc<Vec<WireDartOutputCode>>,
}

pub(crate) fn generate(
    context: WireDartTransferCstGeneratorContext,
    cache: &IrPackComputedCache,
) -> WireDartOutputSpecTransferCstEncoder {
    WireDartOutputSpecTransferCstEncoder {
        api2wire_funcs: cache
            .distinct_input_types
            .iter()
            .map(|ty| generate_api2wire_func(ty, context))
            .collect(),
        api_fill_to_wire_funcs: cache
            .distinct_input_types
            .iter()
            .map(|ty| Acc::new_io(generate_api_fill_to_wire_func(ty, context)))
            .collect(),
    }
}

fn generate_api2wire_func(
    ty: &IrType,
    context: WireDartTransferCstGeneratorContext,
) -> Acc<WireDartOutputCode> {
    let generator = WireDartTransferCstGenerator::new(ty.clone(), context);
    generator
        .api2wire_body()
        .map(|raw_body, target: TargetOrCommon| {
            raw_body
                .map(|raw_body| {
                    let code = format!(
                        "{} api2wire_{}({} raw) {{
                            {raw_body}
                        }}",
                        WireDartTransferCstGenerator::new(ty.clone(), context)
                            .dart_wire_type(target.as_target_or(Target::Io)),
                        ty.safe_ident(),
                        ApiDartGenerator::new(ty.clone(), context.as_api_dart_context())
                            .dart_api_type(),
                    );

                    let (body, api_impl_body) = match target {
                        TargetOrCommon::Common => (code, "".into()),
                        TargetOrCommon::Io | TargetOrCommon::Wasm => {
                            ("".into(), format!("@protected\n{code}"))
                        }
                    };

                    WireDartOutputCode {
                        api_impl_body,
                        body,
                        ..Default::default()
                    }
                })
                .unwrap_or_default()
        })
}

fn generate_api_fill_to_wire_func(
    ty: &IrType,
    context: WireDartGeneratorContext,
) -> WireDartOutputCode {
    if let Some(body) =
        WireDartTransferCstGenerator::new(ty.clone(), context).api_fill_to_wire_body()
    {
        let target_wire_type = match ty {
            Optional(inner) => &inner.inner,
            it => it,
        };

        WireDartOutputCode {
            api_impl_body: format!(
                "void _api_fill_to_wire_{}({} apiObj, {} wireObj) {{
                    {body}
                }}",
                ty.safe_ident(),
                ApiDartGenerator::new(ty.clone(), context.as_api_dart_context()).dart_api_type(),
                WireDartTransferCstGenerator::new(target_wire_type.clone(), context)
                    .dart_wire_type(Target::Io),
            ),
            ..Default::default()
        }
    } else {
        Default::default()
    }
}
