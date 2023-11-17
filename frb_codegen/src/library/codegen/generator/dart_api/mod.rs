pub(super) mod base;
pub(super) mod class;
pub(super) mod decl;
mod function;
mod internal_config;
mod misc;

use crate::codegen::generator::output::dart::DartOutputCode;
use crate::codegen::ir::pack::IrPack;
use anyhow::Result;

pub(crate) fn generate(_ir_pack: &IrPack) -> Result<DartOutputCode> {
    todo!()
}
