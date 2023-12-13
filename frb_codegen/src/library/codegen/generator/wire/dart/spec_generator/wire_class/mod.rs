use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFunc;

mod io;
mod wasm;

pub(super) fn generate(
    config: &GeneratorWireDartInternalConfig,
    c_file_content: &str,
    rust_extern_funcs: &[ExternFunc],
) -> anyhow::Result<Acc<Vec<WireDartOutputCode>>> {
    Ok(Acc {
        io: vec![io::generate(config, c_file_content)?],
        web: vec![wasm::generate(config, rust_extern_funcs)],
        ..Default::default()
    })
}
