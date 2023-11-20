pub(crate) mod internal_config;
mod spec_generator;
mod text_generator;

use crate::codegen::generator::misc::{OutputText, OutputTexts};
use crate::codegen::generator::wire::c::internal_config::GeneratorWireCInternalConfig;
use crate::codegen::ir::pack::IrPack;
use crate::utils::file_utils::create_dir_all_and_write;
use std::path::Path;

pub(crate) struct GeneratorWireCOutput {
    pub output_texts: OutputTexts,
    pub c_file_content: String,
}

pub(crate) fn generate(
    config: &GeneratorWireCInternalConfig,
    extern_func_names: Vec<String>,
    extern_struct_names: Vec<String>,
    rust_output_texts: &OutputTexts,
) -> anyhow::Result<GeneratorWireCOutput> {
    let spec = spec_generator::generate(
        config,
        extern_func_names,
        extern_struct_names,
        rust_output_texts,
    )?;
    let text = text_generator::generate(spec)?;
    Ok(GeneratorWireCOutput {
        output_texts: OutputTexts(vec![OutputText::new(
            config.c_output_path.clone(),
            text.clone(),
        )]),
        c_file_content: text,
    })
}
