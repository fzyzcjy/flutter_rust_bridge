use crate::codegen::dumper::Dumper;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::dart2rust::WireRustOutputSpecWire2api;
use crate::codegen::generator::wire::rust::spec_generator::dump::generate_dump_info;
use crate::codegen::generator::wire::rust::spec_generator::misc::WireRustOutputSpecMisc;
use crate::codegen::generator::wire::rust::spec_generator::rust2dart::WireRustOutputSpecApi2wire;
use crate::codegen::ir::pack::IrPackComputedCache;
use crate::codegen::ConfigDumpContent::GeneratorInfo;
use serde::Serialize;

pub(crate) mod base;
pub(crate) mod dart2rust;
mod dump;
pub(crate) mod extern_func;
pub(crate) mod misc;
pub mod output_code;
pub(crate) mod rust2dart;

#[derive(Serialize)]
pub(super) struct WireRustOutputSpec {
    pub(super) misc: WireRustOutputSpecMisc,
    pub(super) wire2api: WireRustOutputSpecWire2api,
    pub(super) api2wire: WireRustOutputSpecApi2wire,
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
        wire2api: dart2rust::generate(context, &cache),
        api2wire: rust2dart::generate(context, &cache),
    })
}
