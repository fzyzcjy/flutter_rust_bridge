use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::api_dart::base::ApiDartGenerator;
use crate::codegen::generator::wire::dart::spec_generator::base::{
    WireDartGenerator, WireDartGeneratorContext,
};
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::ir::pack::{IrPack, IrPackComputedCache};
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::api_dart::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::generator::wire::dart::spec_generator::wire2api::ty::WireDartGeneratorWire2apiTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;

mod misc;
pub(crate) mod ty;

pub(crate) struct WireDartOutputSpecWire2api {
    impl_wire2api: Acc<Vec<WireDartOutputCode>>,
}

pub(in crate::library::codegen::generator::wire::dart::spec_generator) fn generate(
    context: WireDartGeneratorContext,
    cache: &IrPackComputedCache,
) -> WireDartOutputSpecWire2api {
    WireDartOutputSpecWire2api {
        impl_wire2api: cache
            .distinct_output_types
            .iter()
            .map(|ty| Acc::new_common(generate_impl_wire2api(ty, context)))
            .collect(),
    }
}

fn generate_impl_wire2api(ty: &IrType, context: WireDartGeneratorContext) -> WireDartOutputCode {
    let body = WireDartGenerator::new(ty.clone(), context).generate_impl_wire2api_body();
    WireDartOutputCode(format!(
        "{dart_api_type} _wire2api_{safe_ident}(dynamic raw) {{
            {body}
        }}
        ",
        dart_api_type =
            ApiDartGenerator::new(ty.clone(), context.as_api_dart_context()).dart_api_type(),
        safe_ident = ty.safe_ident(),
    ))
}
