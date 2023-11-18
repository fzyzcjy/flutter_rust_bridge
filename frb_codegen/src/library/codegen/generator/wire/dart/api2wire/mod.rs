use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::api_dart::base::ApiDartGenerator;
use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::dart::base::{WireDartGenerator, WireDartGeneratorContext};
use crate::codegen::generator::wire::dart::output_code::WireDartOutputCode;
use crate::codegen::ir::pack::{IrPack, IrPackComputedCache};
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::Optional;
use crate::library::codegen::generator::api_dart::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::generator::wire::dart::api2wire::ty::WireDartGeneratorApi2wireTrait;
use crate::library::codegen::generator::wire::dart::misc::ty::WireDartGeneratorMiscTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;

pub(crate) mod ty;

pub(crate) struct WireDartOutputSpecApi2wire {
    api2wire_funcs: Acc<Vec<WireDartOutputCode>>,
    api_fill_to_wire_funcs: Acc<Vec<WireDartOutputCode>>,
}

pub(super) fn generate(
    context: WireDartGeneratorContext,
    cache: &IrPackComputedCache,
) -> WireDartOutputSpecApi2wire {
    WireDartOutputSpecApi2wire {
        api2wire_funcs: cache
            .distinct_input_types
            .iter()
            .map(|ty| generate_api2wire_func(ty, context))
            .collect(),
        api_fill_to_wire_funcs: cache
            .distinct_input_types
            .iter()
            .map(|ty| generate_api_fill_to_wire_func(ty, context))
            .collect(),
    }
}

fn generate_api2wire_func(ty: &IrType, context: WireDartGeneratorContext) -> Acc<String> {
    let generator = WireDartGenerator::new(ty.clone(), context);
    generator.api2wire_body().map(|body, target| {
        body.map(|body| {
            format!(
                "@protected
                {} api2wire_{}({} raw) {{
                    {body}
                }}",
                WireDartGenerator::new(ty.clone(), context).dart_wire_type(target),
                ty.safe_ident(),
                ApiDartGenerator::new(ty.clone(), context.as_api_dart_context()).dart_api_type(),
            )
        })
        .unwrap_or_default()
    })
}

fn generate_api_fill_to_wire_func(ty: &IrType, context: WireDartGeneratorContext) -> String {
    if let Some(body) = WireDartGenerator::new(ty.clone(), context).api_fill_to_wire_body() {
        let target_wire_type = match ty {
            Optional(inner) => &inner.inner,
            it => it,
        };

        format!(
            "void _api_fill_to_wire_{}({} apiObj, {} wireObj) {{
                {body}
            }}",
            ty.safe_ident(),
            ApiDartGenerator::new(ty.clone(), context.as_api_dart_context()).dart_api_type(),
            WireDartGenerator::new(target_wire_type.clone(), context).dart_wire_type(Target::Io),
        )
    } else {
        "".to_string()
    }
}
