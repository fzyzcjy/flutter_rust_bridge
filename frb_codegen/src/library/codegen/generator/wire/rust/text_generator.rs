use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::internal_config::GeneratorWireRustInternalConfig;
use crate::codegen::generator::wire::rust::spec_generator::WireRustOutputSpec;
use crate::utils::path_utils::path_to_string;

// Call it "text", not "code", because the whole codegen is generating code,
// and we want to emphasize we are generating final output text here.
pub(super) struct WireRustOutputText {
    common: String,
    io: Option<String>,
    wasm: Option<String>,
}

pub(super) fn generate(
    spec: WireRustOutputSpec,
    config: &GeneratorWireRustInternalConfig,
) -> WireRustOutputText {
    let core_code = generate_core_code(spec);

    let mod_io = emit_platform_module("io", &core_code.io, config, Target::Io)?;
    let mod_wasm = if config.wasm_enabled {
        format!(
            "
            /// cbindgen:ignore
            #[cfg(target_family = \"wasm\")]
            {}
            #[cfg(target_family = \"wasm\")]
            pub use web::*;
            ",
            emit_platform_module("web", &core_code.wasm, config, Target::Wasm)?
        )
    } else {
        "".into()
    };

    let common = format!(
        "{core_code_common}
        {mod_wasm}
        
        #[cfg(not(target_family = \"wasm\"))]
        {mod_io}
        #[cfg(not(target_family = \"wasm\"))]
        pub use io::*;
        ",
        core_code_common = core_code.common,
    );

    let io = (!config.inline_rust).then(|| format!("use super::*;\n{mod_io}"));
    let wasm =
        (!config.inline_rust && config.wasm_enabled).then(|| format!("use super::*;\n{mod_wasm}"));

    WireRustOutputText { common, io, wasm }
}

fn generate_core_code(spec: WireRustOutputSpec) -> Acc<String> {
    todo!()
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
