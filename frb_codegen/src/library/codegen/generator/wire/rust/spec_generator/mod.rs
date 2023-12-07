use crate::codegen::dumper::Dumper;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::WireDartOutputSpecCodecCstDecoder;
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::WireDartOutputSpecCodecDcoEncoder;
use crate::codegen::generator::wire::rust::spec_generator::dump::generate_dump_info;
use crate::codegen::generator::wire::rust::spec_generator::misc::WireRustOutputSpecMisc;
use crate::codegen::ir::pack::IrPackComputedCache;
use crate::codegen::ConfigDumpContent::GeneratorInfo;
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
    pub(super) dart2rust: WireDartOutputSpecCodecCstDecoder,
    pub(super) rust2dart: WireDartOutputSpecCodecDcoEncoder,
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
        dart2rust: codec::cst::decoder::generate(
            context.as_wire_rust_codec_cst_context(),
            &cache.distinct_input_types,
        ),
        rust2dart: codec::dco::encoder::generate(
            context.as_wire_rust_codec_dco_context(),
            &cache.distinct_types,
        ),
    })
}
