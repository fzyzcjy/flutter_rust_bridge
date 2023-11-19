use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::spec_generator::api2wire::WireRustOutputSpecApi2wire;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::misc::WireRustOutputSpecMisc;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::wire2api::WireRustOutputSpecWire2api;
use crate::codegen::ir::pack::{IrPack, IrPackComputedCache};
use crate::codegen::ir::ty::IrType;
use itertools::Itertools;

mod internal_config;
pub(crate) mod spec_generator;
mod writer;

pub(crate) fn generate(ir_pack: &IrPack, context: WireRustGeneratorContext) -> anyhow::Result<()> {
    let spec = spec_generator::generate(ir_pack, context);
    writer::write(spec, context.config)
}
