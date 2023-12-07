use crate::codegen_generator_structs;
use enum_dispatch::enum_dispatch;

codegen_generator_structs!(
    #[enum_dispatch(WireDartTransferSseGeneratorImplTrait)]
    #[enum_dispatch(WireDartTransferSseGeneratorEncoderTrait)]
    #[enum_dispatch(WireDartTransferSseGeneratorDecoderTrait)]
    WireDartTransferSseGenerator
);

#[derive(Debug, Clone, Copy)]
pub(crate) struct WireDartTransferSseGeneratorContext<'a> {
    pub(crate) ir_pack: &'a IrPack,
}
