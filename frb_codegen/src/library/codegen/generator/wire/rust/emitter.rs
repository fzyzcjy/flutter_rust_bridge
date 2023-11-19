use crate::codegen::generator::wire::rust::internal_config::GeneratorWireRustInternalConfig;
use crate::codegen::generator::wire::rust::text_generator::WireRustOutputText;
use std::fs;

pub(super) fn emit(
    text: WireRustOutputText,
    config: &GeneratorWireRustInternalConfig,
) -> anyhow::Result<()> {
    let WireRustOutputText { common, io, wasm } = text;

    fs::write(&config.rust_common_output_path, common)?;
    if let Some(io) = io {
        fs::write(&config.rust_io_output_path, io)?;
    }
    if let Some(wasm) = wasm {
        fs::write(&config.rust_wasm_output_path, wasm)?;
    }

    Ok(())
}
