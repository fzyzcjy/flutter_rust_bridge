use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::codegen::generator::wire::rust::internal_config::GeneratorWireRustInternalConfig;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::ir::ty::IrType::*;
use crate::codegen_generator_structs;
use crate::library::codegen::generator::api_dart::spec_generator::base::ApiDartGeneratorContext;
use enum_dispatch::enum_dispatch;
use paste::paste;

codegen_generator_structs!(
    #[enum_dispatch(WireDartGeneratorImplTrait)]
    #[enum_dispatch(WireDartGeneratorMiscTrait)]
    #[enum_dispatch(WireDartGeneratorDart2RustTrait)]
    #[enum_dispatch(WireDartGeneratorRust2DartTrait)]
    WireDartGenerator
);

#[derive(Debug, Clone, Copy)]
pub(crate) struct WireDartGeneratorContext<'a> {
    pub(crate) ir_pack: &'a IrPack,
    pub(crate) config: &'a GeneratorWireDartInternalConfig,
    pub(crate) wire_rust_config: &'a GeneratorWireRustInternalConfig,
    pub(crate) api_dart_config: &'a GeneratorApiDartInternalConfig,
}

// TODO duplicated with `WireDartTransferCstGeneratorContext`
impl WireDartGeneratorContext<'_> {
    pub(crate) fn as_wire_rust_context(&self) -> WireRustGeneratorContext {
        WireRustGeneratorContext {
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
