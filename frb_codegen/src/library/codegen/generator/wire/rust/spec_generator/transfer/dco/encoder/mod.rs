use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::spec_generator::base::{
    WireRustGenerator, WireRustGeneratorContext,
};
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::base::{
    WireRustCodecDcoGenerator, WireRustCodecDcoGeneratorContext,
};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::IrPackComputedCache;
use crate::library::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::ty::WireRustCodecDcoGeneratorEncoderTrait;
use serde::Serialize;

mod misc;
pub(crate) mod ty;

#[derive(Serialize)]
pub(crate) struct WireDartOutputSpecCodecDcoEncoder {
    pub impl_into_dart: Acc<Vec<WireRustOutputCode>>,
}

pub(crate) fn generate(
    context: WireRustCodecDcoGeneratorContext,
    cache: &IrPackComputedCache,
) -> WireDartOutputSpecCodecDcoEncoder {
    WireDartOutputSpecCodecDcoEncoder {
        impl_into_dart: cache
            .distinct_types
            .iter()
            .filter_map(|ty| {
                WireRustCodecDcoGenerator::new(ty.clone(), context).generate_impl_into_dart()
            })
            .map(|x| Acc::new_common(x.into()))
            .collect(),
    }
}
