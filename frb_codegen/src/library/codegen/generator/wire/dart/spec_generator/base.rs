use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::WireDartCodecCstGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::WireDartCodecDcoGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::codec::sse::base::WireDartCodecSseGeneratorContext;
use crate::codegen::generator::wire::rust::internal_config::GeneratorWireRustInternalConfig;
use crate::codegen::ir::mir::ty::MirType::*;
use crate::codegen_generator_structs;
use crate::library::codegen::generator::api_dart::spec_generator::base::ApiDartGeneratorContext;
use enum_dispatch::enum_dispatch;

codegen_generator_structs!(
    #[enum_dispatch(WireDartGeneratorImplTrait)]
    #[enum_dispatch(WireDartGeneratorMiscTrait)]
    #[enum_dispatch(WireDartGeneratorDart2RustTrait)]
    #[enum_dispatch(WireDartGeneratorRust2DartTrait)]
    WireDartGenerator
);

#[derive(Debug, Clone, Copy)]
pub(crate) struct WireDartGeneratorContext<'a> {
    pub(crate) mir_pack: &'a MirPack,
    pub(crate) config: &'a GeneratorWireDartInternalConfig,
    pub(crate) wire_rust_config: &'a GeneratorWireRustInternalConfig,
    pub(crate) api_dart_config: &'a GeneratorApiDartInternalConfig,
}

// TODO duplicated with `WireDartCodecCstGeneratorContext`
impl WireDartGeneratorContext<'_> {
    pub(crate) fn as_api_dart_context(&self) -> ApiDartGeneratorContext {
        ApiDartGeneratorContext {
            mir_pack: self.mir_pack,
            config: self.api_dart_config,
        }
    }

    pub(crate) fn as_wire_dart_codec_cst_context(&self) -> WireDartCodecCstGeneratorContext {
        WireDartCodecCstGeneratorContext {
            mir_pack: self.mir_pack,
            config: self.config,
            wire_rust_config: self.wire_rust_config,
            api_dart_config: self.api_dart_config,
        }
    }

    pub(crate) fn as_wire_dart_codec_dco_context(&self) -> WireDartCodecDcoGeneratorContext {
        WireDartCodecDcoGeneratorContext {
            mir_pack: self.mir_pack,
            api_dart_config: self.api_dart_config,
        }
    }

    pub(crate) fn as_wire_dart_codec_sse_context(&self) -> WireDartCodecSseGeneratorContext {
        WireDartCodecSseGeneratorContext {
            mir_pack: self.mir_pack,
            api_dart_config: self.api_dart_config,
        }
    }
}
