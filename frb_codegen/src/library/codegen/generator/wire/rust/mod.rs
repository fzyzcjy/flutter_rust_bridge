use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::spec_generator::api2wire::WireRustOutputSpecApi2wire;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::misc::WireRustOutputSpecMisc;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::wire2api::WireRustOutputSpecWire2api;
use crate::codegen::ir::pack::{IrPack, IrPackComputedCache};
use crate::codegen::ir::ty::IrType;
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

    command_run!(
        commands::format_rust,
        &config.rust_output_path,
        (
            config.wasm_enabled,
            config.rust_io_output_path(),
            config.rust_wasm_output_path(),
        )
    )?;

    Ok(())
}
