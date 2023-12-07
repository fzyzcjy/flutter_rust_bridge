use crate::codegen_generator_structs;
use enum_dispatch::enum_dispatch;

codegen_generator_structs!(
    #[enum_dispatch(WireDartTransferDirectGeneratorEncoderTrait)]
    #[enum_dispatch(WireDartTransferDirectGeneratorDecoderTrait)]
    WireDartTransferDirectGenerator
);

#[derive(Debug, Clone, Copy)]
pub(crate) struct WireDartTransferDirectGeneratorContext<'a> {
    pub(crate) ir_pack: &'a IrPack,
}
