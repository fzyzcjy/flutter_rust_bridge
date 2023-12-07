use crate::codegen_generator_structs;
use enum_dispatch::enum_dispatch;

codegen_generator_structs!(
    #[enum_dispatch(WireRustTransferSseGeneratorImplTrait)]
    #[enum_dispatch(WireRustTransferSseGeneratorEncoderTrait)]
    #[enum_dispatch(WireRustTransferSseGeneratorDecoderTrait)]
    WireRustTransferSseGenerator
);

#[derive(Debug, Clone, Copy)]
pub(crate) struct WireRustTransferSseGeneratorContext<'a> {
    pub(crate) ir_pack: &'a IrPack,
}
