use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::dart::api2wire::WireDartOutputSpecApi2wire;
use crate::codegen::generator::wire::dart::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::misc::WireDartOutputSpecMisc;
use crate::codegen::generator::wire::dart::output_code::WireDartOutputCode;
use crate::codegen::generator::wire::dart::wire2api::WireDartOutputSpecWire2api;
use crate::codegen::ir::pack::{IrPack, IrPackComputedCache};

mod api2wire;
pub(crate) mod base;
mod internal_config;
pub(crate) mod misc;
mod output_code;
mod wire2api;

pub(crate) struct WireDartOutputSpec {
    misc: WireDartOutputSpecMisc,
    wire2api: WireDartOutputSpecWire2api,
    api2wire: WireDartOutputSpecApi2wire,
}

pub(crate) fn generate(ir_pack: &IrPack, context: WireDartGeneratorContext) -> WireDartOutputSpec {
    let cache = IrPackComputedCache::compute(ir_pack);
    WireDartOutputSpec {
        misc: misc::generate(context, &cache),
        wire2api: wire2api::generate(context, &cache),
        api2wire: api2wire::generate(context, &cache),
    }
}
