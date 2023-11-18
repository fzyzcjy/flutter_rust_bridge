use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::dart::base::{WireDartGenerator, WireDartGeneratorContext};
use crate::codegen::generator::wire::dart::output_code::WireDartOutputCode;
use crate::codegen::ir::pack::{IrPack, IrPackComputedCache};
use crate::codegen::ir::ty::IrType;
use itertools::Itertools;

pub(crate) mod ty;

pub(crate) struct WireDartOutputSpecWire2api {
    impl_wire2api: Acc<Vec<WireDartOutputCode>>,
}

pub(super) fn generate(
    context: WireDartGeneratorContext,
    cache: &IrPackComputedCache,
) -> WireDartOutputSpecWire2api {
    WireDartOutputSpecWire2api {
        impl_wire2api: cache
            .distinct_output_types
            .iter()
            .map(|ty| generate_impl_wire2api(ty, context))
            .collect(),
    }
}

fn generate_impl_wire2api(ty: &IrType, context: WireDartGeneratorContext) -> String {
    let body = WireDartGenerator::new(ty.clone(), context).wire2api_body();
    format!(
        "{} _wire2api_{}(dynamic raw) {{
            {body}
        }}
        ",
        ty.dart_api_type(),
        ty.safe_ident(),
    )
}
