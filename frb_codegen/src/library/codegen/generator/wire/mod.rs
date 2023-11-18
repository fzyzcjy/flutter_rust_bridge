mod c;
mod dart;
mod misc;
mod rust;

use crate::codegen::generator::output::OutputCode;
use crate::codegen::ir::pack::IrPack;
use anyhow::Result;

pub(crate) fn generate(ir_pack: &IrPack) -> Result<OutputCode> {
    rust::generate(ir_pack, context);
    c::generate(ir_pack);
    dart::generate(ir_pack, context);
    Ok(TODO)
}
