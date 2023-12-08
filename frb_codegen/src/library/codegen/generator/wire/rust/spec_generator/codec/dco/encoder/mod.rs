use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::spec_generator::base::{
    WireRustGenerator, WireRustGeneratorContext,
};
use crate::codegen::generator::wire::rust::spec_generator::codec::base::WireRustCodecOutputSpec;
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::base::{
    WireRustCodecDcoGenerator, WireRustCodecDcoGeneratorContext,
};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::IrPackComputedCache;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::ty::WireRustCodecDcoGeneratorEncoderTrait;
use serde::Serialize;

mod misc;
pub(crate) mod ty;

pub(crate) fn generate(
    context: WireRustCodecDcoGeneratorContext,
    types: &[IrType],
) -> WireRustCodecOutputSpec {
    WireRustCodecOutputSpec {
        inner: (types.iter())
            .filter_map(|ty| {
                WireRustCodecDcoGenerator::new(ty.clone(), context).generate_impl_into_dart()
            })
            .map(|x| Acc::<WireRustOutputCode>::new_common(x.into()))
            .collect(),
    }
}
