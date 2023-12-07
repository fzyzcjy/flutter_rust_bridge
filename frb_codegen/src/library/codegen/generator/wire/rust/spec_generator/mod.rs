use crate::codegen::dumper::Dumper;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::dart2rust::WireDartOutputSpecTransferCstDecoder;
use crate::codegen::generator::wire::rust::spec_generator::dump::generate_dump_info;
use crate::codegen::generator::wire::rust::spec_generator::misc::WireRustOutputSpecMisc;
use crate::codegen::generator::wire::rust::spec_generator::rust2dart::WireDartOutputSpecTransferDcoEncoder;
use crate::codegen::ir::pack::IrPackComputedCache;
use crate::codegen::ConfigDumpContent::GeneratorInfo;
use serde::Serialize;

pub(crate) mod base;
mod dump;
pub(crate) mod extern_func;
pub(crate) mod misc;
pub mod output_code;
pub(crate) mod transfer;

#[derive(Serialize)]
pub(super) struct WireRustOutputSpec {
    pub(super) misc: WireRustOutputSpecMisc,
    pub(super) dart2rust: WireDartOutputSpecTransferCstDecoder,
    pub(super) rust2dart: WireDartOutputSpecTransferDcoEncoder,
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
        dart2rust: dart2rust::generate(context, &cache),
        rust2dart: rust2dart::generate(context, &cache),
    })
}
