use crate::codegen::generator::wire::dart::spec_generator::api2wire::WireDartOutputSpecApi2wire;
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::misc::WireDartOutputSpecMisc;
use crate::codegen::generator::wire::dart::spec_generator::wire2api::WireDartOutputSpecWire2api;
use crate::codegen::ir::pack::{IrPack, IrPackComputedCache};

pub mod api2wire;
pub(crate) mod base;
pub(crate) mod misc;
mod output_code;
pub mod wire2api;

pub(crate) struct WireDartOutputSpec {
    pub(super) misc: WireDartOutputSpecMisc,
    wire2api: WireDartOutputSpecWire2api,
    api2wire: WireDartOutputSpecApi2wire,
}

pub(crate) fn generate(context: WireDartGeneratorContext) -> WireDartOutputSpec {
    let cache = IrPackComputedCache::compute(context.ir_pack);
    WireDartOutputSpec {
        misc: misc::generate(context, &cache),
        wire2api: wire2api::generate(context, &cache),
        api2wire: api2wire::generate(context, &cache),
    }
}
