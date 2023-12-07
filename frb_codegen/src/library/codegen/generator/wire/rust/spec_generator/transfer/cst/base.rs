use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::base::WireDartCodecCstGeneratorContext;
use crate::codegen::generator::wire::rust::internal_config::GeneratorWireRustInternalConfig;
use crate::codegen_generator_structs;
use enum_dispatch::enum_dispatch;

codegen_generator_structs!(
    #[enum_dispatch(WireRustCodecCstGeneratorImplTrait)]
    #[enum_dispatch(WireRustCodecCstGeneratorEncoderTrait)]
    #[enum_dispatch(WireRustCodecCstGeneratorDecoderTrait)]
    WireRustCodecCstGenerator
);

#[derive(Debug, Clone, Copy)]
pub(crate) struct WireRustCodecCstGeneratorContext<'a> {
    pub(crate) ir_pack: &'a IrPack,
    pub(crate) config: &'a GeneratorWireRustInternalConfig,
    pub(crate) wire_dart_config: &'a GeneratorWireDartInternalConfig,
    pub(crate) api_dart_config: &'a GeneratorApiDartInternalConfig,
}

impl WireRustCodecCstGeneratorContext<'_> {
    pub(crate) fn as_wire_dart_context(&self) -> WireDartCodecCstGeneratorContext {
        WireDartCodecCstGeneratorContext {
            ir_pack: self.ir_pack,
            config: self.wire_dart_config,
            wire_rust_config: self.config,
            api_dart_config: self.api_dart_config,
        }
    }
}
