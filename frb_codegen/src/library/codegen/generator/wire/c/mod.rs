mod cbindgen_executor;
mod dummy_function;
pub(crate) mod internal_config;

use crate::codegen::generator::wire::c::internal_config::GeneratorWireCInternalConfig;
use crate::codegen::ir::pack::IrPack;
use crate::library::commands::cbindgen::{cbindgen, CbindgenArgs};
use crate::utils::file_utils::{create_dir_all_and_write, temp_change_file};
use std::path::{Path, PathBuf};

// TODO
// fn get_c_struct_names(ir_pack: &IrPack) -> Vec<String> {
//     ir_pack
//         .distinct_types(true, true)
//         .iter()
//         .filter_map(|ty| {
//             if let IrType::StructRef(_) = ty {
//                 Some(ty.rust_wire_type(Target::Io))
//             } else {
//                 None
//             }
//         })
//         .collect()
// }

pub(crate) fn generate(
    ir_pack: &IrPack,
    config: &GeneratorWireCInternalConfig,
) -> anyhow::Result<()> {
    let code_cbindgen = cbindgen_executor::execute(ir_pack, config)?;
    let code_dummy = dummy_function::generate(config);
    emit(code_cbindgen, &code_dummy, &config.c_output_path)?;
    Ok(())
}

fn emit(code_cbindgen: String, code_dummy: &str, c_output_path: &Path) -> anyhow::Result<()> {
    let text = code_cbindgen + code_dummy;
    Ok(create_dir_all_and_write(c_output_path, text)?)
}
