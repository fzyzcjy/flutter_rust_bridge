use crate::codegen::config::internal_config::GeneratorInternalConfig;
use crate::codegen::dumper::Dumper;
use crate::codegen::generator::misc::path_texts::PathTexts;
use crate::codegen::ir::mir::pack::MirPack;
use crate::codegen::misc::GeneratorProgressBarPack;

pub(crate) mod acc;
pub(crate) mod api_dart;
pub(crate) mod codec;
pub(crate) mod misc;
pub(crate) mod wire;

pub(crate) struct GeneratorOutput {
    pub output_texts: PathTexts,
    pub dart_needs_freezed: bool,
}

pub(crate) fn generate(
    mir_pack: &MirPack,
    config: &GeneratorInternalConfig,
    dumper: &Dumper,
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<GeneratorOutput> {
    let api_dart_output = api_dart::generate(mir_pack, &config.api_dart, dumper)?;
    let wire_output = wire::generate(
        mir_pack,
        &config.wire,
        &config.api_dart,
        &api_dart_output.output_texts.paths(),
        dumper,
        progress_bar_pack,
    )?;

    let output_texts = wire_output.output_texts + api_dart_output.output_texts;
    let output_texts = output_texts.merge();

    Ok(GeneratorOutput {
        output_texts,
        dart_needs_freezed: api_dart_output.needs_freezed,
    })
}
