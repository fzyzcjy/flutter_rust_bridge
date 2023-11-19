mod cbindgen_executor;
mod dummy_function;

use crate::codegen::ir::pack::IrPack;
use crate::library::commands::cbindgen::{cbindgen, CbindgenArgs};
use crate::utils::file_utils::temp_change_file;
use std::path::PathBuf;

// TODO unify with `GeneratorCInternalConfig` config
pub(crate) struct Config {
    pub(crate) rust_crate_dir: PathBuf,
    pub(crate) rust_output_path: PathBuf,
}

pub(crate) fn generate(ir_pack: &IrPack, config: &Config) -> anyhow::Result<()> {
    let code_cbindgen = cbindgen_executor::execute(ir_pack, config)?;
    let code_dummy = dummy_function::generate_dummy_function(todo!());
    Ok(())
}
