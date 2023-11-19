use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::internal_config::GeneratorWireRustInternalConfig;
use crate::codegen::generator::wire::rust::spec_generator::WireRustOutputSpec;
use crate::utils::path_utils::path_to_string;

// Call it "text", not "code", because the whole codegen is generating code,
// and we want to emphasize we are generating final output text here.
pub(super) struct WireRustOutputText {
    pub(super) text: Acc<Option<String>>,
}

pub(super) fn generate(
    spec: WireRustOutputSpec,
    config: &GeneratorWireRustInternalConfig,
) -> WireRustOutputText {
    let core_code = generate_core_code_from_spec(spec);
    generate_text_from_core_code(config, &core_code)
}

fn generate_core_code_from_spec(spec: WireRustOutputSpec) -> Acc<String> {
    todo!()
}

fn generate_text_from_core_code(
    config: &GeneratorWireRustInternalConfig,
    core_code: &Acc<String>,
) -> WireRustOutputText {
    WireRustOutputText {
        text: Acc {
            common: Some(generate_text_common(&core_code.common)),
            io: Some(generate_text_target(&core_code.io)),
            wasm: (config.wasm_enabled).then(|| generate_text_target(&core_code.wasm)),
        },
    }
}

fn generate_text_common(
    core_code_common: &str,
    config: &GeneratorWireRustInternalConfig,
) -> String {
    let mod_io = generate_text_common_mod_declaration("io", config, Target::Io)?;

    let mod_wasm = if config.wasm_enabled {
        generate_text_common_mod_declaration("web", config, Target::Wasm)?
    } else {
        "".into()
    };

    format!(
        "{core_code_common}
        {mod_io}
        {mod_wasm}
        ",
    );
}

fn generate_text_common_mod_declaration(
    name: &str,
    config: &GeneratorWireRustInternalConfig,
    target: Target,
) -> anyhow::Result<String> {
    let prelude = match target {
        Target::Io => "",
        Target::Wasm => "/// cbindgen:ignore",
    };

    let cfg = match target {
        Target::Io => r#"not(target_family = "wasm")"#,
        Target::Wasm => r#"(target_family = "wasm")"#,
    };

    let mod_path = path_to_string(&config.rust_output_path[target.into()])?;

    Ok(format!(
        "
        {prelude}
        #[cfg({cfg})]
        #[path = \"{mod_path}\"]
        mod {name};
        #[cfg({cfg})]
        pub use {name}::*;
        "
    ))
}

fn generate_text_target(core_code_target: &str) -> String {
    format!("use super::*;\n{core_code_target}")
}
