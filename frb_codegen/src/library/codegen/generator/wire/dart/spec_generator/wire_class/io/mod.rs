pub(super) mod common;
mod via_ffigen;
mod via_manual;

use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFunc;
use crate::codegen::misc::GeneratorProgressBarPack;

pub(crate) fn generate(
    config: &GeneratorWireDartInternalConfig,
    c_file_content: &str,
    rust_extern_funcs: &[ExternFunc],
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<WireDartOutputCode> {
    if config.has_ffigen {
        via_ffigen::generate(config, c_file_content, progress_bar_pack)
    } else {
        via_manual::generate(config, rust_extern_funcs)
    }
}
