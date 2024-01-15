use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFunc;

pub(crate) fn generate(
    config: &GeneratorWireDartInternalConfig,
    rust_extern_funcs: &[ExternFunc],
) -> anyhow::Result<WireDartOutputCode> {
    TODO
}
