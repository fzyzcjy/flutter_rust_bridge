use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::api_dart::spec_generator::base::ApiDartGeneratorContext;
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
    pub(crate) mir_pack: &'a MirPack,
    pub(crate) api_dart_config: &'a GeneratorApiDartInternalConfig,
}

impl WireDartCodecSseGeneratorContext<'_> {
    pub(crate) fn as_api_dart_context(&self) -> ApiDartGeneratorContext {
        ApiDartGeneratorContext {
            mir_pack: self.mir_pack,
            config: self.api_dart_config,
        }
    }
}
