use crate::codegen::generator::acc::Acc;
use crate::library::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;

use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::{
    WireRustCodecCstGenerator, WireRustCodecCstGeneratorContext,
};

use crate::codegen::generator::wire::rust::spec_generator::codec::base::WireRustCodecOutputSpec;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::impl_new_with_nullptr::generate_impl_new_with_nullptr;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::impl_decode_trait::generate_impl_decode;
use crate::codegen::ir::mir::ty::MirType;

mod impl_decode_trait;
mod impl_new_with_nullptr;
mod misc;
pub(crate) mod ty;

pub(crate) fn generate(
    context: WireRustCodecCstGeneratorContext,
    types: &[MirType],
) -> WireRustCodecOutputSpec {
    let mut inner = Default::default();
    inner += (types.iter())
        .map(|ty| WireRustCodecCstGenerator::new(ty.clone(), context).generate_allocate_funcs())
        .collect();
    inner += generate_impl_decode(types, context);
    inner += Acc::new_io(
        (types.iter())
            .filter_map(|ty| {
                WireRustCodecCstGenerator::new(ty.clone(), context).generate_decoder_class()
            })
            .collect(),
    );
    inner += Acc::new_io(generate_impl_new_with_nullptr(types, context));
    WireRustCodecOutputSpec { inner }
}
