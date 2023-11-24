use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;

mod io;
mod wasm;

pub(super) fn generate(
    config: &GeneratorWireDartInternalConfig,
    c_file_content: &str,
) -> anyhow::Result<Acc<Vec<WireDartOutputCode>>> {
    Ok(Acc {
        io: vec![io::generate(config, c_file_content)?],
        wasm: vec![wasm::generate(config)],
        ..Default::default()
    })
}
