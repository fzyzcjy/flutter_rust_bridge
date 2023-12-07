use crate::library::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;
use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::spec_generator::base::{
    WireRustGenerator, WireRustGeneratorContext,
};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::{
    WireRustCodecCstGenerator, WireRustCodecCstGeneratorContext,
};
use crate::codegen::generator::wire::rust::IrPackComputedCache;
use serde::Serialize;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::impl_new_with_nullptr::generate_impl_new_with_nullptr;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::impl_decode_trait::generate_impl_decode;
use crate::codegen::ir::ty::IrType;

mod impl_decode_trait;
mod impl_new_with_nullptr;
mod misc;
pub(crate) mod ty;

#[derive(Serialize)]
pub(crate) struct WireDartOutputSpecCodecCstDecoder {
    pub allocate_funcs: Acc<Vec<WireRustOutputCode>>,
    pub impl_decode: Acc<Vec<WireRustOutputCode>>,
    pub decoder_class: Acc<Vec<WireRustOutputCode>>,
    pub impl_new_with_nullptr: Acc<Vec<WireRustOutputCode>>,
}

pub(crate) fn generate(
    context: WireRustCodecCstGeneratorContext,
    types: &[IrType],
) -> WireDartOutputSpecCodecCstDecoder {
    WireDartOutputSpecCodecCstDecoder {
        allocate_funcs: (types.iter())
            .map(|ty| WireRustCodecCstGenerator::new(ty.clone(), context).generate_allocate_funcs())
            .collect(),
        impl_decode: generate_impl_decode(&types, context),
        decoder_class: Acc::new_io(
            (types.iter())
                .filter_map(|ty| {
                    WireRustCodecCstGenerator::new(ty.clone(), context).generate_decoder_class()
                })
                .map(|x| x.into())
                .collect(),
        ),
        impl_new_with_nullptr: Acc::new_io(generate_impl_new_with_nullptr(types, context)),
    }
}
