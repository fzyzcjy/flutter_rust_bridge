use crate::codegen_generator_structs;
use enum_dispatch::enum_dispatch;

codegen_generator_structs!(
    #[enum_dispatch(WireRustTransferDirectGeneratorEncoderTrait)]
    // #[enum_dispatch(WireRustTransferDirectGeneratorDecoderTrait)] // TODO
    WireRustTransferDirectGenerator
);

#[derive(Debug, Clone, Copy)]
pub(crate) struct WireRustTransferDirectGeneratorContext<'a> {
    pub(crate) ir_pack: &'a IrPack,
}
