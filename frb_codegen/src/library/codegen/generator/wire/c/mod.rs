pub(crate) mod internal_config;
mod spec_generator;
mod text_generator;

use crate::codegen::generator::misc::OutputTexts;
use crate::codegen::generator::wire::c::internal_config::GeneratorWireCInternalConfig;
use crate::codegen::ir::pack::IrPack;
use crate::utils::file_utils::create_dir_all_and_write;
use std::path::Path;

pub(crate) struct WireCOutputPack {
    pub output_texts: OutputTexts,
    pub c_file_content: String,
}

pub(crate) fn generate(
    _ir_pack: &IrPack,
    config: &GeneratorWireCInternalConfig,
    extern_func_names: Vec<String>,
    extern_struct_names: Vec<String>,
) -> anyhow::Result<WireCOutputPack> {
    let spec = spec_generator::generate(config, extern_func_names, extern_struct_names)?;
    let text = text_generator::generate(spec)?;
    Ok(WireCOutputPack {
        c_file_content: text,
    })
}
