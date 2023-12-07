use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::api_dart::spec_generator::base::ApiDartGeneratorContext;
use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::codegen::generator::wire::rust::internal_config::GeneratorWireRustInternalConfig;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::base::WireRustCodecCstGeneratorContext;
use crate::codegen_generator_structs;
use enum_dispatch::enum_dispatch;

codegen_generator_structs!(
    #[enum_dispatch(WireDartCodecCstGeneratorImplTrait)]
    #[enum_dispatch(WireDartCodecCstGeneratorEncoderTrait)]
    WireDartCodecCstGenerator
);

#[derive(Debug, Clone, Copy)]
pub(crate) struct WireDartCodecCstGeneratorContext<'a> {
    pub(crate) ir_pack: &'a IrPack,
    pub(crate) config: &'a GeneratorWireDartInternalConfig,
    pub(crate) wire_rust_config: &'a GeneratorWireRustInternalConfig,
    pub(crate) api_dart_config: &'a GeneratorApiDartInternalConfig,
}

impl WireDartCodecCstGeneratorContext<'_> {
    pub(crate) fn as_wire_rust_context(&self) -> WireRustCodecCstGeneratorContext {
        WireRustCodecCstGeneratorContext {
            ir_pack: self.ir_pack,
            config: self.wire_rust_config,
            wire_dart_config: self.config,
            api_dart_config: self.api_dart_config,
        }
    }

    pub(crate) fn as_api_dart_context(&self) -> ApiDartGeneratorContext {
        ApiDartGeneratorContext {
            ir_pack: self.ir_pack,
            config: self.api_dart_config,
        }
    }
}
