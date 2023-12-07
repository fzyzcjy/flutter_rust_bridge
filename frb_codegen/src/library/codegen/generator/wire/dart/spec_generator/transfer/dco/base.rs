use crate::codegen_generator_structs;
use enum_dispatch::enum_dispatch;

codegen_generator_structs!(
    #[enum_dispatch(WireDartTransferDcoGeneratorImplTrait)]
    #[enum_dispatch(WireDartTransferDcoGeneratorEncoderTrait)]
    #[enum_dispatch(WireDartTransferDcoGeneratorDecoderTrait)]
    WireDartTransferDcoGenerator
);

#[derive(Debug, Clone, Copy)]
pub(crate) struct WireDartTransferDcoGeneratorContext<'a> {
    pub(crate) ir_pack: &'a IrPack,
}
