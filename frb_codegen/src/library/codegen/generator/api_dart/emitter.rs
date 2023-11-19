use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::utils::file_utils::create_dir_all_and_write;

pub(super) fn emit(text: String, config: &GeneratorApiDartInternalConfig) -> anyhow::Result<()> {
    // TODO handle multi file
    let path = config.dart_decl_output_path.values().next().unwrap();
    create_dir_all_and_write(path, text)
}
