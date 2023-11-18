use crate::codegen::config::internal_config::GeneratorInternalConfig;
use crate::codegen::generator::output::OutputCode;
use crate::codegen::ir::pack::IrPack;

mod acc;
pub(crate) mod api_dart;
mod misc;
pub(crate) mod output;
pub(crate) mod wire;

pub(crate) fn generate(ir_pack: &IrPack, config: &GeneratorInternalConfig) -> anyhow::Result<()> {
    // TODO seems not this ideal. we need to call various external tools that directly write to fs
    let output_code_dart_api: OutputCode =
        api_dart::generate(&ir_pack, &config.dart.clone().into())?.into();
    let output_code_wire = wire::generate(&ir_pack)?;
    let output_code = output_code_dart_api.merge(output_code_wire);
    output_code.write()?;
    Ok(())
}
