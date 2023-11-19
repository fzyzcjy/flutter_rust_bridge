pub(crate) mod c;
pub(crate) mod dart;
pub(crate) mod misc;
pub(crate) mod rust;

use crate::codegen::config::internal_config::GeneratorWireInternalConfig;
use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::codegen::generator::wire::rust::internal_config::GeneratorWireRustInternalConfig;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::ir::pack::IrPack;
use anyhow::Result;

pub(crate) fn generate(
    ir_pack: &IrPack,
    config: &GeneratorWireInternalConfig,
    api_dart_config: &GeneratorApiDartInternalConfig,
) -> Result<()> {
    // TODO more systematic approach
    let context = WireRustGeneratorContext {
        ir_pack,
        config: &GeneratorWireRustInternalConfig {},
        wire_dart_config: &GeneratorWireDartInternalConfig {},
        api_dart_config,
    };

    rust::generate(todo!())?;
    c::generate(ir_pack, todo!())?;
    dart::generate(todo!())?;
    Ok(())
}
