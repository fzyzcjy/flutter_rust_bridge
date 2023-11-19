use crate::codegen::generator::misc::{write_code_for_targets, TargetOrCommon};
use crate::codegen::generator::wire::rust::internal_config::GeneratorWireRustInternalConfig;
use crate::codegen::generator::wire::rust::text_generator::WireRustOutputText;
use crate::utils::file_utils::create_dir_all_and_write;
use strum::IntoEnumIterator;

pub(super) fn emit(
    text: &WireRustOutputText,
    config: &GeneratorWireRustInternalConfig,
) -> anyhow::Result<()> {
    write_code_for_targets(&text.text, &config.rust_output_path)
}
