use crate::codegen_generator_structs;
use enum_dispatch::enum_dispatch;

codegen_generator_structs!(
    #[enum_dispatch(WireRustCodecDcoGeneratorImplTrait)]
    #[enum_dispatch(WireRustCodecDcoGeneratorEncoderTrait)]
    #[enum_dispatch(WireRustCodecDcoGeneratorDecoderTrait)]
    WireRustCodecDcoGenerator
);

#[derive(Debug, Clone, Copy)]
pub(crate) struct WireRustCodecDcoGeneratorContext<'a> {
    pub(crate) mir_pack: &'a MirPack,
}
