use crate::codegen::generator::wire::c::internal_config::GeneratorWireCInternalConfig;
use crate::utils::file_utils::create_dir_all_and_write;

pub(super) fn emit(text: &str, config: &GeneratorWireCInternalConfig) -> anyhow::Result<()> {
    create_dir_all_and_write(&config.c_output_path, text)
}
