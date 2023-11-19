mod dummy_function;

use crate::codegen::ir::pack::IrPack;
use crate::library::commands::cbindgen::{cbindgen, CbindgenArgs};

pub(crate) fn generate(ir_pack: &IrPack) -> anyhow::Result<()> {
    todo!()
}

fn execute_cbindgen() -> anyhow::Result<()> {
    cbindgen(CbindgenArgs {
        rust_crate_dir: TODO,
        c_output_path: TODO,
        c_struct_names: TODO,
        exclude_symbols: TODO,
    })?;
}
