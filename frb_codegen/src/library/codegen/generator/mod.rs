use crate::codegen::config::internal_config::GeneratorInternalConfig;
use crate::codegen::ir::pack::IrPack;

pub(crate) mod acc;
pub(crate) mod api_dart;
pub(crate) mod misc;
pub(crate) mod wire;

pub(crate) struct GeneratorOutput {
    pub dart_needs_freezed: bool,
}

pub(crate) fn generate(
    ir_pack: &IrPack,
    config: &GeneratorInternalConfig,
) -> anyhow::Result<GeneratorOutput> {
    api_dart::generate(&ir_pack, &config.api_dart)?;
    let wire_output = wire::generate(&ir_pack, &config.wire, &config.api_dart)?;

    Ok(GeneratorOutput {
        dart_needs_freezed: wire_output.dart_needs_freezed,
    })
}
