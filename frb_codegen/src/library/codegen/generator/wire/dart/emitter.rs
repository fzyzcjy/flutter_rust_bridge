use crate::codegen::generator::misc::{write_code_for_targets, TargetOrCommon};
use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::codegen::generator::wire::dart::text_generator::WireDartOutputText;

pub(super) fn emit(
    text: WireDartOutputText,
    config: &GeneratorWireDartInternalConfig,
) -> anyhow::Result<()> {
    write_code_for_targets(&text.text, &config.dart_impl_output_path)
}
