use crate::codegen::dumper::Dumper;
use crate::codegen::generator::misc::codec::CodecMode;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::codec::base::{
    WireRustCodecEntrypoint, WireRustCodecOutputSpec,
};
use crate::codegen::generator::wire::rust::spec_generator::dump::generate_dump_info;
use crate::codegen::generator::wire::rust::spec_generator::misc::WireRustOutputSpecMisc;
use crate::codegen::ir::pack::IrPackComputedCache;
use crate::codegen::ConfigDumpContent::GeneratorInfo;
use itertools::Itertools;
use serde::Serialize;
use strum::IntoEnumIterator;

pub(crate) mod base;
pub(crate) mod codec;
mod dump;
pub(crate) mod extern_func;
pub(crate) mod misc;
pub mod output_code;

#[derive(Serialize)]
pub(super) struct WireRustOutputSpec {
    pub(super) misc: WireRustOutputSpecMisc,
    pub(super) dart2rust: WireRustCodecOutputSpec,
    pub(super) rust2dart: WireRustCodecOutputSpec,
}

pub(super) fn generate(
    context: WireRustGeneratorContext,
    dumper: &Dumper,
) -> anyhow::Result<WireRustOutputSpec> {
    let cache = IrPackComputedCache::compute(context.ir_pack);

    dumper.dump(
        GeneratorInfo,
        "wire_rust.json",
        &generate_dump_info(&cache, context),
    )?;

    Ok(WireRustOutputSpec {
        misc: misc::generate(context, &cache)?,
        dart2rust: CodecMode::iter()
            .map(WireRustCodecEntrypoint::from)
            .flat_map(|codec| codec.generate_decode(context, &cache.distinct_input_types))
            .collect(),
        rust2dart: CodecMode::iter()
            .map(WireRustCodecEntrypoint::from)
            .flat_map(|codec| codec.generate_encode(context, &cache.distinct_types))
            .collect(),
    })
}
