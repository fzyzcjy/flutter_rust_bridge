use crate::codegen_generator_structs;
use enum_dispatch::enum_dispatch;

codegen_generator_structs!(
    #[enum_dispatch(WireRustTransferDcoGeneratorImplTrait)]
    #[enum_dispatch(WireRustTransferDcoGeneratorEncoderTrait)]
    #[enum_dispatch(WireRustTransferDcoGeneratorDecoderTrait)]
    WireRustTransferDcoGenerator
);

#[derive(Debug, Clone, Copy)]
pub(crate) struct WireRustTransferDcoGeneratorContext<'a> {
    pub(crate) ir_pack: &'a IrPack,
}
