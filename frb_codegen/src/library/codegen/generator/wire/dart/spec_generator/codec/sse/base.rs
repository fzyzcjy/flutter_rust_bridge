use crate::codegen_generator_structs;
use enum_dispatch::enum_dispatch;

codegen_generator_structs!(
    #[enum_dispatch(WireDartCodecSseGeneratorImplTrait)]
    #[enum_dispatch(WireDartCodecSseGeneratorEncoderTrait)]
    #[enum_dispatch(WireDartCodecSseGeneratorDecoderTrait)]
    WireDartCodecSseGenerator
);

#[derive(Debug, Clone, Copy)]
pub(crate) struct WireDartCodecSseGeneratorContext<'a> {
    pub(crate) ir_pack: &'a IrPack,
}
