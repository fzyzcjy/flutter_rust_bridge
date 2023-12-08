use crate::codegen::dumper::Dumper;
use crate::codegen::generator::misc::codec::CodecMode;
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::codec::base::{
    WireDartCodecEntrypoint, WireDartCodecOutputSpec,
};
use crate::codegen::generator::wire::dart::spec_generator::dump::generate_dump_info;
use crate::codegen::generator::wire::dart::spec_generator::misc::WireDartOutputSpecMisc;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFunc;
use crate::codegen::ir::pack::IrPackComputedCache;
use crate::codegen::ConfigDumpContent::GeneratorInfo;
use itertools::Itertools;
use serde::Serialize;
use std::path::PathBuf;
use strum::IntoEnumIterator;

pub(crate) mod base;
pub(crate) mod codec;
mod dump;
pub(crate) mod misc;
pub(crate) mod output_code;
pub(super) mod wire_class;

#[derive(Clone, Serialize)]
pub(crate) struct WireDartOutputSpec {
    pub(super) misc: WireDartOutputSpecMisc,
    pub(super) rust2dart: WireDartCodecOutputSpec,
    pub(super) dart2rust: WireDartCodecOutputSpec,
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
        rust2dart: CodecMode::iter()
            .map(WireDartCodecEntrypoint::from)
            .flat_map(|codec| codec.generate_decode(context, &cache.distinct_output_types))
            .collect(),
        dart2rust: CodecMode::iter()
            .map(WireDartCodecEntrypoint::from)
            .flat_map(|codec| codec.generate_encode(context, &cache.distinct_input_types))
            .collect(),
    })
}
