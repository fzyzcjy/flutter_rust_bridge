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
    let core_code = generate_core_code(spec);

    WireRustOutputText {
        text: Acc {
            common: Some(generate_common(&core_code.common)),
            io: Some(generate_target(&core_code.io)),
            wasm: (config.wasm_enabled).then(|| generate_target(&core_code.wasm)),
        },
    }
}

fn generate_core_code(spec: WireRustOutputSpec) -> Acc<String> {
    todo!()
}

fn generate_common(core_code_common: &str, config: &GeneratorWireRustInternalConfig) -> String {
    let mod_io = format!(
        "
        #[cfg(not(target_family = \"wasm\"))]
        {}
        #[cfg(not(target_family = \"wasm\"))]
        pub use io::*;
        ",
        generate_mod_declaration("io", config, Target::Io)?
    );

    let mod_wasm = if config.wasm_enabled {
        format!(
            "
            /// cbindgen:ignore
            #[cfg(target_family = \"wasm\")]
            {}
            #[cfg(target_family = \"wasm\")]
            pub use web::*;
            ",
            generate_mod_declaration("web", config, Target::Wasm)?
        )
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

fn generate_target(core_code_target: &str) -> String {
    format!("use super::*;\n{core_code_target}")
}

fn generate_mod_declaration(
    name: &str,
    config: &GeneratorWireRustInternalConfig,
    target: Target,
) -> anyhow::Result<String> {
    let path = path_to_string(&config.rust_output_path[target.into()])?;
    Ok(format!("#[path = \"{path}\"] mod {name};"))
}
