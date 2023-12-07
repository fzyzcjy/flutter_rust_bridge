use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::spec_generator::codec::sse::base::WireRustCodecSseGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::pack::IrPackComputedCache;
use crate::codegen::ir::ty::IrType;
use serde::Serialize;

pub(crate) mod ty;

#[derive(Serialize)]
pub(crate) struct WireDartOutputSpecCodecSseDecoder {
    pub impl_decode: Acc<Vec<WireRustOutputCode>>,
}

pub(crate) fn generate(
    context: WireRustCodecSseGeneratorContext,
    cache: &IrPackComputedCache,
) -> WireDartOutputSpecCodecSseDecoder {
    WireDartOutputSpecCodecSseDecoder {
        impl_decode: generate_impl_decode(&cache.distinct_input_types, context),
    }
}

pub(crate) fn generate_impl_decode(
    types: &[IrType],
    context: WireRustCodecSseGeneratorContext,
) -> Acc<Vec<WireRustOutputCode>> {
    todo!()
}
