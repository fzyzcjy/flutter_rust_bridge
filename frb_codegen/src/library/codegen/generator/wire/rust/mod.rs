use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::api2wire::WireRustOutputSpecApi2wire;
use crate::codegen::generator::wire::rust::base::WireRustGeneratorContext;
use crate::codegen::generator::wire::rust::misc::WireRustOutputSpecMisc;
use crate::codegen::generator::wire::rust::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::wire2api::WireRustOutputSpecWire2api;
use crate::codegen::ir::pack::{IrPack, IrPackComputedCache};
use crate::codegen::ir::ty::IrType;
use itertools::Itertools;

pub(crate) mod api2wire;
pub(crate) mod base;
pub(in crate::library::codegen::generator::wire::rust) mod extern_func;
mod internal_config;
pub(crate) mod misc;
mod output_code;
pub(crate) mod wire2api;

pub(crate) struct WireRustOutputSpec {
    misc: WireRustOutputSpecMisc,
    wire2api: WireRustOutputSpecWire2api,
    api2wire: WireRustOutputSpecApi2wire,
}

pub(crate) fn generate(ir_pack: &IrPack, context: WireRustGeneratorContext) -> anyhow::Result<()> {
    let spec = generate_spec(ir_pack, context);
    todo!();
    Ok(())
}

fn generate_spec(ir_pack: &IrPack, context: WireRustGeneratorContext) -> WireRustOutputSpec {
    let cache = IrPackComputedCache::compute(ir_pack);
    WireRustOutputSpec {
        misc: misc::generate(context, &cache),
        wire2api: wire2api::generate(context, &cache),
        api2wire: api2wire::generate(context, &cache),
    }
}
