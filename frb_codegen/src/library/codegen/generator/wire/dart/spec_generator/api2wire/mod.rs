use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::dart::spec_generator::base::{
    WireDartGenerator, WireDartGeneratorContext,
};
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::ir::pack::IrPackComputedCache;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::Optional;
use crate::library::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::generator::wire::dart::spec_generator::api2wire::ty::WireDartGeneratorApi2wireTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;

mod misc;
pub(crate) mod ty;

pub(crate) struct WireDartOutputSpecApi2wire {
    api2wire_funcs: Acc<Vec<WireDartOutputCode>>,
    api_fill_to_wire_funcs: Acc<Vec<WireDartOutputCode>>,
}

pub(crate) fn generate(
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
            .map(|ty| Acc::new_io(generate_api_fill_to_wire_func(ty, context)))
            .collect(),
    }
}

fn generate_api2wire_func(
    ty: &IrType,
    context: WireDartGeneratorContext,
) -> Acc<WireDartOutputCode> {
    let generator = WireDartGenerator::new(ty.clone(), context);
    generator.api2wire_body().map(|body, target| {
        let target = target.to_target_or(Target::Io);
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
            .into()
        })
        .unwrap_or_default()
    })
}

fn generate_api_fill_to_wire_func(
    ty: &IrType,
    context: WireDartGeneratorContext,
) -> WireDartOutputCode {
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
        .into()
    } else {
        Default::default()
    }
}
