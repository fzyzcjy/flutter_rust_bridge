use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::spec_generator::WireRustOutputSpec;

// Call it "text", not "code", because the whole codegen is generating code,
// and we want to emphasize we are generating final output text here.
pub(super) struct WireRustOutputText {
    common: String,
    io: Option<String>,
    wasm: Option<String>,
}

pub(super) fn generate(spec: WireRustOutputSpec) -> WireRustOutputText {
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

    if !config.inline_rust {
        let io = format!("use super::*;\n{io}");
        if config.wasm_enabled {
            let wasm = format!("use super::*;\n{wasm}");
        }
    }

    WireRustOutputText {}
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
