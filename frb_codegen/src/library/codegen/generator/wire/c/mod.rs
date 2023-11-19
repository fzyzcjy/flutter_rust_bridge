mod dummy_function;

use crate::codegen::ir::pack::IrPack;
use crate::library::commands::cbindgen::{cbindgen, CbindgenArgs};

pub(crate) fn generate(ir_pack: &IrPack) -> anyhow::Result<()> {
    execute_cbindgen()?;
    todo!()
}

fn execute_cbindgen() -> anyhow::Result<String> {
    cbindgen(CbindgenArgs {
        rust_crate_dir: todo!(),
        c_struct_names: todo!(),
        exclude_symbols: todo!(),
    })?;
}
