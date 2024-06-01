use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen_generator_structs;
use enum_dispatch::enum_dispatch;

codegen_generator_structs!(
    #[enum_dispatch(WireRustCodecSseGeneratorImplTrait)]
    #[enum_dispatch(WireRustCodecSseGeneratorEncoderTrait)]
    #[enum_dispatch(WireRustCodecSseGeneratorDecoderTrait)]
    WireRustCodecSseGenerator
);

#[derive(Debug, Clone, Copy)]
pub(crate) struct WireRustCodecSseGeneratorContext<'a> {
    pub(crate) mir_pack: &'a MirPack,
    pub(crate) api_dart_config: &'a GeneratorApiDartInternalConfig,
}
