use crate::codegen::dumper::Dumper;
use crate::codegen::generator::codec::structs::EncodeOrDecode::{Decode, Encode};
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::codec::base::{
    WireRustCodecEntrypoint, WireRustCodecOutputSpec,
};
use crate::codegen::generator::wire::rust::spec_generator::dump::generate_dump_info;
use crate::codegen::generator::wire::rust::spec_generator::misc::WireRustOutputSpecMisc;
use crate::codegen::ir::mir::pack::MirPackComputedCache;
use crate::codegen::ConfigDumpContent::GeneratorInfo;
use itertools::Itertools;
use serde::Serialize;

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
    pub(super) extern_struct_names: Vec<String>,
}

pub(super) fn generate(
    context: WireRustGeneratorContext,
    dumper: &Dumper,
) -> anyhow::Result<WireRustOutputSpec> {
    let cache = MirPackComputedCache::compute(context.mir_pack);

    (dumper.with_content(GeneratorInfo))
        .dump("wire_rust.json", &generate_dump_info(&cache, context))?;

    let dart2rust = WireRustCodecEntrypoint::generate_all(context, &cache, Decode);
    let rust2dart = WireRustCodecEntrypoint::generate_all(context, &cache, Encode);
    let extern_struct_names = generate_extern_struct_names(&dart2rust, &rust2dart);

    Ok(WireRustOutputSpec {
        misc: misc::generate(context, &cache)?,
        dart2rust,
        rust2dart,
        extern_struct_names,
    })
}

fn generate_extern_struct_names(
    dart2rust: &WireRustCodecOutputSpec,
    rust2dart: &WireRustCodecOutputSpec,
) -> Vec<String> {
    [dart2rust.inner.io.clone(), rust2dart.inner.io.clone()]
        .concat()
        .iter()
        .flat_map(|x| x.extern_classes.clone())
        .map(|x| x.name)
        .unique()
        .collect_vec()
}
