pub(crate) mod c;
pub(crate) mod dart;
pub(crate) mod misc;
pub(crate) mod rust;

use crate::codegen::config::internal_config::GeneratorWireInternalConfig;
use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::misc::PathTexts;
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::ir::pack::IrPack;
use anyhow::Result;

pub(crate) struct GeneratorWireOutput {
    pub output_texts: PathTexts,
    pub dart_needs_freezed: bool,
}

pub(crate) fn generate(
    ir_pack: &IrPack,
    config: &GeneratorWireInternalConfig,
    api_dart_config: &GeneratorApiDartInternalConfig,
) -> Result<GeneratorWireOutput> {
    let wire_rust_generator_context = WireRustGeneratorContext {
        ir_pack,
        config: &config.rust,
        wire_dart_config: &config.dart,
        api_dart_config,
    };
    let wire_dart_generator_context = WireDartGeneratorContext {
        ir_pack,
        config: &config.dart,
        wire_rust_config: &config.rust,
        api_dart_config,
    };

    let rust_output = rust::generate(wire_rust_generator_context)?;

    let c_output = c::generate(
        &config.c,
        rust_output.extern_func_names,
        rust_output.extern_struct_names,
        &rust_output.output_texts,
    )?;

    let dart_output = dart::generate(wire_dart_generator_context, &c_output.c_file_content)?;

    Ok(GeneratorWireOutput {
        output_texts: rust_output.output_texts + c_output.output_texts + dart_output.output_texts,
        dart_needs_freezed: dart_output.dart_needs_freezed,
    })
}
