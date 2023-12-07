use crate::codegen_generator_structs;
use enum_dispatch::enum_dispatch;

codegen_generator_structs!(
    #[enum_dispatch(WireDartTransferCstGeneratorImplTrait)]
    #[enum_dispatch(WireDartTransferCstGeneratorEncoderTrait)]
    WireDartTransferCstGenerator
);

#[derive(Debug, Clone, Copy)]
pub(crate) struct WireDartTransferCstGeneratorContext<'a> {
    pub(crate) ir_pack: &'a IrPack,
}
