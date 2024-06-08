use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::spec_generator::codec::base::WireRustCodecOutputSpec;
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::base::{
    WireRustCodecDcoGenerator, WireRustCodecDcoGeneratorContext,
};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::mir::ty::MirType;
use crate::library::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::ty::WireRustCodecDcoGeneratorEncoderTrait;

mod misc;
pub(crate) mod ty;

pub(crate) fn generate(
    context: WireRustCodecDcoGeneratorContext,
    types: &[MirType],
) -> WireRustCodecOutputSpec {
    let mut inner = Default::default();
    // inner += generate_misc();
    inner += (types.iter())
        .filter_map(|ty| {
            WireRustCodecDcoGenerator::new(ty.clone(), context).generate_impl_into_dart()
        })
        .map(|x| Acc::<WireRustOutputCode>::new_common(x.into()))
        .collect();
    WireRustCodecOutputSpec { inner }
}
