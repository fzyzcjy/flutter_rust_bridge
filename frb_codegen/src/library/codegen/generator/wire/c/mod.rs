pub(crate) mod internal_config;
mod spec_generator;
mod text_generator;

use crate::codegen::dumper::internal_config::ConfigDumpContent;
use crate::codegen::dumper::Dumper;
use crate::codegen::generator::misc::path_texts::{PathText, PathTexts};
use crate::codegen::generator::wire::c::internal_config::GeneratorWireCInternalConfig;
use crate::codegen::misc::GeneratorProgressBarPack;
use crate::utils::basic_code::general_code::GeneralCode;

pub(crate) struct GeneratorWireCOutput {
    pub output_texts: PathTexts,
    pub c_file_content: String,
}

pub(crate) fn generate(
    config: &GeneratorWireCInternalConfig,
    extern_func_names: Vec<String>,
    extern_struct_names: Vec<String>,
    rust_output_texts: &PathTexts,
    dumper: &Dumper,
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<GeneratorWireCOutput> {
    let spec = spec_generator::generate(
        config,
        extern_func_names,
        extern_struct_names,
        rust_output_texts,
        progress_bar_pack,
    )?;
    (dumper.with_content(ConfigDumpContent::GeneratorSpec)).dump("wire_c.json", &spec)?;

    let text = text_generator::generate(spec)?;
    (dumper.with_content(ConfigDumpContent::GeneratorText)).dump_str("wire_c/content.h", &text)?;

    Ok(GeneratorWireCOutput {
        output_texts: PathTexts({
            let mut ans = vec![];
            if let Some(c_output_path) = &config.c_output_path {
                ans.push(PathText::new(
                    c_output_path.clone(),
                    GeneralCode::new_c(text.clone()),
                ))
            }
            ans
        }),
        c_file_content: text,
    })
}
