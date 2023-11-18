use crate::codegen::generator::wire::rust::WireRustOutputSpec;

pub(crate) fn write(spec: WireRustOutputSpec) -> anyhow::Result<()> {
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
pub use io::*;
",
        common,
        mod_web = if config.wasm_enabled {
            format!(
                "
/// cbindgen:ignore
#[cfg(target_family = \"wasm\")]
{}
#[cfg(target_family = \"wasm\")]
pub use web::*;",
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
