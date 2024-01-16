use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFunc;
use crate::codegen::misc::GeneratorProgressBarPack;

mod io;
mod web;

pub(super) fn generate(
    config: &GeneratorWireDartInternalConfig,
    c_file_content: &str,
    rust_extern_funcs: &[ExternFunc],
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<Acc<Vec<WireDartOutputCode>>> {
    Ok(Acc {
        io: vec![io::generate(
            config,
            c_file_content,
            rust_extern_funcs,
            progress_bar_pack,
        )?],
        web: vec![web::generate(config, rust_extern_funcs)],
        ..Default::default()
    })
}
