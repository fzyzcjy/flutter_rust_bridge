use crate::codegen::generator::misc::TargetOrCommon;
use crate::codegen::generator::wire::rust::internal_config::GeneratorWireRustInternalConfig;
use crate::codegen::generator::wire::rust::text_generator::WireRustOutputText;
use std::fs;
use strum::IntoEnumIterator;

pub(super) fn emit(
    text: WireRustOutputText,
    config: &GeneratorWireRustInternalConfig,
) -> anyhow::Result<()> {
    for target in TargetOrCommon::iter() {
        if let Some(text) = &text.text[target] {
            fs::write(&config.rust_output_path[target], text)?;
        }
    }
    Ok(())
}
