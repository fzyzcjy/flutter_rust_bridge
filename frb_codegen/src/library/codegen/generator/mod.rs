use crate::codegen::config::internal_config::GeneratorInternalConfig;
use crate::codegen::generator::misc::PathTexts;
use crate::codegen::ir::pack::IrPack;

pub(crate) mod acc;
pub(crate) mod api_dart;
pub(crate) mod misc;
pub(crate) mod wire;

pub(crate) struct GeneratorOutput {
    pub output_texts: PathTexts,
    pub dart_needs_freezed: bool,
}

pub(crate) fn generate(
    ir_pack: &IrPack,
    config: &GeneratorInternalConfig,
) -> anyhow::Result<GeneratorOutput> {
    let api_dart_output = api_dart::generate(&ir_pack, &config.api_dart)?;
    let wire_output = wire::generate(&ir_pack, &config.wire, &config.api_dart)?;

    Ok(GeneratorOutput {
        output_texts: api_dart_output.output_texts + wire_output.output_texts,
        dart_needs_freezed: wire_output.dart_needs_freezed,
    })
}
