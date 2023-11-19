use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::internal_config::GeneratorWireRustInternalConfig;
use crate::codegen::generator::wire::rust::spec_generator::WireRustOutputSpec;
use crate::codegen::generator::wire::rust::text_generator::WireRustOutputText;
use crate::utils::path_utils::path_to_string;
use std::fs;

pub(super) fn emit(
    spec: WireRustOutputText,
    config: &GeneratorWireRustInternalConfig,
) -> anyhow::Result<()> {
    let Acc { common, io, wasm } = &generated_rust.code;

    let mod_web = if config.wasm_enabled {
        format!(
            "
    /// cbindgen:ignore
    #[cfg(target_family = \"wasm\")]
    {}
    #[cfg(target_family = \"wasm\")]
    pub use web::*;",
            emit_platform_module("web", wasm, config, Target::Wasm)?
        )
    } else {
        "".into()
    };

    let mod_io = emit_platform_module("io", io, config, Target::Io);

    let common = format!(
        "{common}
    {mod_web}
    
    #[cfg(not(target_family = \"wasm\"))]
    {mod_io}
    #[cfg(not(target_family = \"wasm\"))]
    pub use io::*;
    "
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

fn emit_platform_module(
    name: &str,
    body: &str,
    config: &GeneratorWireRustInternalConfig,
    target: Target,
) -> anyhow::Result<String> {
    Ok(if config.inline_rust {
        format!("mod {name} {{ use super::*;\n {body} }}")
    } else {
        let path = match target {
            Target::Io => config.rust_io_output_path(),
            Target::Wasm => config.rust_wasm_output_path(),
        };
        let path = path_to_string(path)?;
        format!("#[path = \"{path}\"] mod {name};")
    })
}
