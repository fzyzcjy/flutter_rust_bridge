pub(crate) mod internal_config;
mod spec_generator;
mod text_generator;

use crate::codegen::dumper::internal_config::ConfigDumpContent;
use crate::codegen::dumper::Dumper;
use crate::codegen::generator::misc::path_texts::{PathText, PathTexts};
use crate::codegen::generator::wire::c::internal_config::GeneratorWireCInternalConfig;
use crate::codegen::misc::GeneratorProgressBarPack;

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
    dumper.dump(ConfigDumpContent::GeneratorSpec, "wire_c.json", &spec)?;

    let text = text_generator::generate(spec)?;
    dumper.dump_str(ConfigDumpContent::GeneratorText, "wire_c/content.h", &text)?;

    Ok(GeneratorWireCOutput {
        output_texts: PathTexts({
            let mut ans = vec![];
            if let Some(c_output_path) = &config.c_output_path {
                ans.push(PathText::new(c_output_path.clone(), text.clone()))
            }
            ans
        }),
        c_file_content: text,
    })
}
