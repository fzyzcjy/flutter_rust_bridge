pub(crate) mod internal_config;
mod spec_generator;
mod text_generator;

use crate::codegen::generator::wire::c::internal_config::GeneratorWireCInternalConfig;
use crate::codegen::ir::pack::IrPack;
use crate::utils::file_utils::create_dir_all_and_write;
use std::path::Path;

pub(crate) struct WireCOutputPack {
    pub c_file_content: String,
}

pub(crate) fn generate(
    _ir_pack: &IrPack,
    config: &GeneratorWireCInternalConfig,
    // TODO originally from: `generated_rust.extern_func_names`
    extern_func_names: Vec<String>,
    // TODO originally created via `get_c_struct_names`, should calc it from wire-rust layer, in analogy to `extern_func_names`
    extern_struct_names: Vec<String>,
) -> anyhow::Result<WireCOutputPack> {
    let spec = spec_generator::generate(config, extern_func_names, extern_struct_names)?;
    let text = text_generator::generate(spec)?;
    Ok(WireCOutputPack {
        c_file_content: text,
    })
}

fn emit(code_cbindgen: String, code_dummy: &str, c_output_path: &Path) -> anyhow::Result<()> {
    let text = code_cbindgen + code_dummy;
    Ok(create_dir_all_and_write(c_output_path, text)?)
}
