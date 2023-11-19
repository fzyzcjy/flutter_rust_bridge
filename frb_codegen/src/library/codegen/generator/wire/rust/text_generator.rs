use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::{Target, TargetOrCommon};
use crate::codegen::generator::misc::text_generator_utils::section_header_comment;
use crate::codegen::generator::wire::rust::internal_config::GeneratorWireRustInternalConfig;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::WireRustOutputSpec;
use crate::utils::path_utils::path_to_string;
use itertools::Itertools;
use strum::IntoEnumIterator;

// Call it "text", not "code", because the whole codegen is generating code,
// and we want to emphasize we are generating final output text here.
pub(super) struct WireRustOutputText {
    pub(super) text: Acc<Option<String>>,
    pub(super) extern_func_names: Vec<String>,
}

pub(super) fn generate(
    spec: &WireRustOutputSpec,
    config: &GeneratorWireRustInternalConfig,
) -> anyhow::Result<WireRustOutputText> {
    let merged_code = generate_merged_code(&spec);
    let text = generate_text_from_merged_code(
        config,
        &merged_code.clone().map(|code, _| code.all_code()),
    )?;
    let extern_func_names = compute_extern_func_names(merged_code);

    Ok(WireRustOutputText {
        text,
        extern_func_names,
    })
}

fn compute_extern_func_names(merged_code: Acc<WireRustOutputCode>) -> Vec<String> {
    let extern_funcs_acc = merged_code.map(|code, _| code.extern_funcs);
    let extern_funcs = TargetOrCommon::iter().flat_map(|target| extern_funcs_acc[target].clone());
    (extern_funcs.map(|extern_func| extern_func.func_name)).collect_vec()
}

fn generate_merged_code(spec: &WireRustOutputSpec) -> Acc<WireRustOutputCode> {
    let mut merged_code = Acc::<Vec<WireRustOutputCode>>::default();
    let mut add = |section_name: &str, item: &Acc<Vec<WireRustOutputCode>>| {
        merged_code.push(section_header_comment(section_name).into());
        merged_code += item.clone();
    };

    add("file_attributes", &spec.misc.file_attributes);
    add("code_header", &spec.misc.code_header);
    add("imports", &spec.misc.imports);
    add("wire_funcs", &spec.misc.wire_funcs);
    add("wrapper_structs", &spec.misc.wrapper_structs);
    add("static_checks", &spec.misc.static_checks);
    add("executor", &spec.misc.executor);
    add("allocate_funcs", &spec.wire2api.allocate_funcs);
    add("related_funcs", &spec.wire2api.related_funcs);
    add("impl_wire2api", &spec.wire2api.impl_wire2api);
    add("wire2api_class", &spec.wire2api.wire2api_class);
    add(
        "impl_new_with_nullptr",
        &spec.wire2api.impl_new_with_nullptr,
    );
    add("impl_into_dart", &spec.api2wire.impl_into_dart);

    merged_code.map(|code, _| code.into_iter().fold(Default::default(), |a, b| a + b))
}

fn generate_text_from_merged_code(
    config: &GeneratorWireRustInternalConfig,
    core_code: &Acc<String>,
) -> anyhow::Result<Acc<Option<String>>> {
    Ok(Acc {
        common: Some(generate_text_common(&core_code.common, config)?),
        io: Some(generate_text_target(&core_code.io)),
        wasm: (config.wasm_enabled).then(|| generate_text_target(&core_code.wasm)),
    })
}

fn generate_text_common(
    core_code_common: &str,
    config: &GeneratorWireRustInternalConfig,
) -> anyhow::Result<String> {
    let mod_io = generate_text_common_mod_declaration("io", config, Target::Io)?;

    let mod_wasm = if config.wasm_enabled {
        generate_text_common_mod_declaration("web", config, Target::Wasm)?
    } else {
        "".into()
    };

    Ok(format!(
        "{core_code_common}
        {mod_io}
        {mod_wasm}
        ",
    ))
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
