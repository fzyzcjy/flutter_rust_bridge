use crate::codegen::dumper::Dumper;
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::dump::generate_dump_info;
use crate::codegen::generator::wire::dart::spec_generator::misc::WireDartOutputSpecMisc;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::encoder::WireDartOutputSpecCodecCstEncoder;
use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::decoder::WireDartOutputSpecCodecDcoDecoder;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFunc;
use crate::codegen::ir::pack::IrPackComputedCache;
use crate::codegen::ConfigDumpContent::GeneratorInfo;
use serde::Serialize;
use std::path::PathBuf;

pub(crate) mod base;
mod dump;
pub(crate) mod misc;
pub(crate) mod output_code;
pub(crate) mod transfer;
pub(super) mod wire_class;

#[derive(Clone, Serialize)]
pub(crate) struct WireDartOutputSpec {
    pub(super) misc: WireDartOutputSpecMisc,
    pub(super) rust2dart: WireDartOutputSpecCodecDcoDecoder,
    pub(super) dart2rust: WireDartOutputSpecCodecCstEncoder,
}

pub(crate) fn generate(
    context: WireDartGeneratorContext,
    c_file_content: &str,
    api_dart_actual_output_paths: &[PathBuf],
    rust_extern_funcs: &[ExternFunc],
    dumper: &Dumper,
) -> anyhow::Result<WireDartOutputSpec> {
    let cache = IrPackComputedCache::compute(context.ir_pack);

    dumper.dump(
        GeneratorInfo,
        "wire_dart.json",
        &generate_dump_info(&cache, context),
    )?;

    Ok(WireDartOutputSpec {
        misc: misc::generate(
            context,
            &cache,
            c_file_content,
            api_dart_actual_output_paths,
            rust_extern_funcs,
        )?,
        rust2dart: transfer::dco::decoder::generate(
            context.as_wire_dart_transfer_dco_context(),
            &cache,
        ),
        dart2rust: transfer::cst::encoder::generate(
            context.as_wire_dart_transfer_cst_context(),
            &cache,
        ),
    })
}
