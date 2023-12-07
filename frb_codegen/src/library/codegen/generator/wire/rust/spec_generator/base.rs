use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::rust::internal_config::GeneratorWireRustInternalConfig;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::base::WireRustTransferCstGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::transfer::dco::base::WireRustTransferDcoGeneratorContext;
use crate::codegen::ir::ty::IrType::*;
use crate::codegen_generator_structs;
use enum_dispatch::enum_dispatch;
use paste::paste;

codegen_generator_structs!(
    #[enum_dispatch(WireRustGeneratorImplTrait)]
    #[enum_dispatch(WireRustGeneratorRust2DartTrait)]
    #[enum_dispatch(WireRustGeneratorDart2RustTrait)]
    #[enum_dispatch(WireRustGeneratorMiscTrait)]
    WireRustGenerator
);

#[derive(Debug, Clone, Copy)]
pub(crate) struct WireRustGeneratorContext<'a> {
    pub(crate) ir_pack: &'a IrPack,
    pub(crate) config: &'a GeneratorWireRustInternalConfig,
    pub(crate) wire_dart_config: &'a GeneratorWireDartInternalConfig,
    pub(crate) api_dart_config: &'a GeneratorApiDartInternalConfig,
}

impl WireRustGeneratorContext<'_> {
    pub(crate) fn as_wire_dart_context(&self) -> WireDartGeneratorContext {
        WireDartGeneratorContext {
            ir_pack: self.ir_pack,
            config: self.wire_dart_config,
            wire_rust_config: self.config,
            api_dart_config: self.api_dart_config,
        }
    }

    pub(crate) fn as_wire_rust_transfer_cst_context(&self) -> WireRustTransferCstGeneratorContext {
        WireRustTransferCstGeneratorContext {
            ir_pack: self.ir_pack,
            config: self.config,
            wire_dart_config: self.wire_dart_config,
            api_dart_config: self.api_dart_config,
        }
    }

    pub(crate) fn as_wire_rust_transfer_dco_context(&self) -> WireRustTransferDcoGeneratorContext {
        WireRustTransferDcoGeneratorContext {
            ir_pack: self.ir_pack,
        }
    }
}
