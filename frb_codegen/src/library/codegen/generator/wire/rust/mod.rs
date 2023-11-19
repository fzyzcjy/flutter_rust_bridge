use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::TargetOrCommon;
use crate::codegen::generator::wire::rust::internal_config::GeneratorWireRustInternalConfig;
use crate::codegen::generator::wire::rust::spec_generator::api2wire::WireRustOutputSpecApi2wire;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::misc::WireRustOutputSpecMisc;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::wire2api::WireRustOutputSpecWire2api;
use crate::codegen::ir::pack::{IrPack, IrPackComputedCache};
use crate::codegen::ir::ty::IrType;
use crate::command_run;
use crate::library::commands::format_rust::format_rust;
use itertools::Itertools;

mod add_mod_to_lib;
mod emitter;
mod internal_config;
pub(crate) mod spec_generator;
mod text_generator;

pub(crate) fn generate(ir_pack: &IrPack, context: WireRustGeneratorContext) -> anyhow::Result<()> {
    let rust_output_dir = Path::new(&rust_output_paths.base_path).parent().unwrap();
    fs::create_dir_all(rust_output_dir)?;

    let spec = spec_generator::generate(ir_pack, context);
    let text = text_generator::generate(spec, context.config)?;
    emitter::emit(text, context.config)?;

    if !config.skip_add_mod_to_lib {
        others::try_add_mod_to_lib(&config.rust_crate_dir, &config.rust_output_path);
    }

    execute_format_rust(&context.config)?;

    Ok(())
}

fn execute_format_rust(config: &GeneratorWireRustInternalConfig) -> anyhow::Result<()> {
    command_run!(
        format_rust,
        &config.rust_output_path[TargetOrCommon::Common],
        &config.rust_output_path[TargetOrCommon::Io],
        (
            config.wasm_enabled,
            &config.rust_output_path[TargetOrCommon::Wasm],
        )
    )
}
