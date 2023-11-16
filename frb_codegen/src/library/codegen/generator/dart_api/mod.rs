pub(super) mod base;
pub(super) mod class;
pub(super) mod decl;
mod internal_config;

use crate::codegen::generator::output::OutputCode;
use crate::codegen::ir::pack::IrPack;
use anyhow::Result;

pub(crate) fn generate(_ir_pack: &IrPack) -> Result<OutputCode> {
    todo!()
}
