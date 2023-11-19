mod c;
mod dart;
mod misc;
mod rust;

use crate::codegen::config::internal_config::GeneratorWireInternalConfig;
use crate::codegen::ir::pack::IrPack;
use anyhow::Result;

pub(crate) fn generate(ir_pack: &IrPack, config: &GeneratorWireInternalConfig) -> Result<()> {
    rust::generate(todo!())?;
    c::generate(ir_pack, todo!())?;
    dart::generate(todo!())?;
    Ok(())
}
