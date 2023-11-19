use crate::commands;
use crate::target::{Acc, Target};
use crate::{command_run, generator, ir, others, Opts};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

pub(crate) fn generate_rust_code(
    config: &Opts,
    ir_pack: &ir::IrPack,
) -> anyhow::Result<generator::rust::Output> {
    let rust_output_paths = config.get_rust_output_paths();

    let rust_output_dir = Path::new(&rust_output_paths.base_path).parent().unwrap();
    fs::create_dir_all(rust_output_dir)?;

    // DONE
    // let generated_rust = ir_pack.generate_rust(config);
    // write_rust_modules(config, &generated_rust)?;

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

    Ok(generated_rust)
}
