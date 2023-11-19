use crate::codegen::config::internal_config::GeneratorInternalConfig;
use crate::codegen::ir::pack::IrPack;

mod acc;
pub(crate) mod api_dart;
mod misc;
pub(crate) mod wire;

pub(crate) fn generate(ir_pack: &IrPack, config: &GeneratorInternalConfig) -> anyhow::Result<()> {
    api_dart::generate(&ir_pack, &config.api_dart)?;
    wire::generate(&ir_pack, &config.wire)?;
    Ok(())
}
