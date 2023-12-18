use crate::commands;
use crate::target::{Acc, Target};
use crate::{command_run, generator, ir, others, Opts};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

pub(crate) fn generate_rust_code(
    config: &Opts,
    ir_file: &ir::IrFile,
) -> anyhow::Result<generator::rust::Output> {
    let rust_output_paths = config.get_rust_output_paths();

    let rust_output_dir = Path::new(&rust_output_paths.base_path).parent().unwrap();
    fs::create_dir_all(rust_output_dir)?;

    let generated_rust = ir_file.generate_rust(config);
    write_rust_modules(config, &generated_rust)?;

    if !config.skip_add_mod_to_lib {
        others::try_add_mod_to_lib(&config.rust_crate_dir, &config.rust_output_path);
    }

    command_run!(
        commands::format_rust,
        &config.rust_output_path,
        (
            config.wasm_enabled && !config.inline_rust,
            config.rust_io_output_path(),
            config.rust_wasm_output_path(),
        )
    )?;

    Ok(generated_rust)
}

fn write_rust_modules(
    config: &Opts,
    generated_rust: &generator::rust::Output,
) -> anyhow::Result<()> {
    let Acc { common, io, wasm } = &generated_rust.code;
    fn emit_platform_module(name: &str, body: &str, config: &Opts, target: Target) -> String {
        if config.inline_rust {
            format!("mod {name} {{ use super::*;\n {body} }}")
        } else {
            let path = match target {
                Target::Io => config.rust_io_output_path(),
                Target::Wasm => config.rust_wasm_output_path(),
                _ => panic!("unsupported target: {:?}", target),
            };
            let path = path.file_name().and_then(OsStr::to_str).unwrap();
            format!("#[path = \"{path}\"] mod {name};")
        }
    }
    let common = format!(
        "{}
{mod_web}

#[cfg(not(target_family = \"wasm\"))]
{mod_io}
#[cfg(not(target_family = \"wasm\"))]
pub use self::io::*;
",
        common,
        mod_web = if config.wasm_enabled {
            format!(
                "
/// cbindgen:ignore
#[cfg(target_family = \"wasm\")]
{}
#[cfg(target_family = \"wasm\")]
pub use self::web::*;",
                emit_platform_module("web", wasm, config, Target::Wasm)
            )
        } else {
            "".into()
        },
        mod_io = emit_platform_module("io", io, config, Target::Io),
    );
    fs::write(&config.rust_output_path, common)?;
    if !config.inline_rust {
        fs::write(config.rust_io_output_path(), format!("use super::*;\n{io}"))?;
        if config.wasm_enabled {
            fs::write(
                config.rust_wasm_output_path(),
                format!("use super::*;\n{wasm}"),
            )?;
        }
    }
    Ok(())
}
