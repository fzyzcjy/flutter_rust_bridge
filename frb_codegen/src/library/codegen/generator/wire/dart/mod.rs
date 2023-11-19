use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::ir::pack::IrPack;

pub(crate) mod internal_config;
pub(super) mod spec_generator;

pub(crate) fn generate(ir_pack: &IrPack, context: WireDartGeneratorContext) -> anyhow::Result<()> {
    let spec = spec_generator::generate(ir_pack, context);
    todo!()
}
