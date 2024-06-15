pub(crate) mod c;
pub(crate) mod dart;
pub(crate) mod misc;
pub(crate) mod rust;

use crate::codegen::config::internal_config::GeneratorWireInternalConfig;
use crate::codegen::dumper::Dumper;
use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::misc::path_texts::PathTexts;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::ir::mir::pack::MirPack;
use crate::codegen::misc::GeneratorProgressBarPack;
use anyhow::Result;
use itertools::Itertools;
use std::path::PathBuf;

pub(crate) struct GeneratorWireOutput {
    pub output_texts: PathTexts,
}

pub(crate) fn generate(
    mir_pack: &MirPack,
    config: &GeneratorWireInternalConfig,
    api_dart_config: &GeneratorApiDartInternalConfig,
    api_dart_actual_output_paths: &[PathBuf],
    dumper: &Dumper,
    progress_bar_pack: &GeneratorProgressBarPack,
) -> Result<GeneratorWireOutput> {
    let wire_rust_generator_context = WireRustGeneratorContext {
        mir_pack,
        config: &config.rust,
        wire_dart_config: &config.dart,
        api_dart_config,
    };
    let wire_dart_generator_context = WireDartGeneratorContext {
        mir_pack,
        config: &config.dart,
        wire_rust_config: &config.rust,
        api_dart_config,
    };

    let rust_output = rust::generate(wire_rust_generator_context, dumper)?;

    let c_output = c::generate(
        &config.c,
        (rust_output.extern_funcs.iter())
            .filter(|x| x.target == Target::Io)
            .map(|x| x.func_name(&config.c.c_symbol_prefix))
            .collect_vec(),
        rust_output.extern_struct_names,
        &rust_output.output_texts,
        dumper,
        progress_bar_pack,
    )?;

    let dart_output = dart::generate(
        wire_dart_generator_context,
        &c_output.c_file_content,
        api_dart_actual_output_paths,
        &rust_output.extern_funcs,
        rust_output.content_hash,
        dumper,
        progress_bar_pack,
    )?;

    Ok(GeneratorWireOutput {
        output_texts: rust_output.output_texts + c_output.output_texts + dart_output.output_texts,
    })
}
