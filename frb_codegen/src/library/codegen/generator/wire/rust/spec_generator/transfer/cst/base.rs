use crate::codegen_generator_structs;
use enum_dispatch::enum_dispatch;

codegen_generator_structs!(
    #[enum_dispatch(WireRustTransferCstGeneratorImplTrait)]
    #[enum_dispatch(WireRustTransferCstGeneratorEncoderTrait)]
    #[enum_dispatch(WireRustTransferCstGeneratorDecoderTrait)]
    WireRustTransferCstGenerator
);

#[derive(Debug, Clone, Copy)]
pub(crate) struct WireRustTransferCstGeneratorContext<'a> {
    pub(crate) ir_pack: &'a IrPack,
}
