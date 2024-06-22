use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::{Target, TargetOrCommon};
use crate::codegen::generator::misc::text_generator_utils::{
    generate_text_respecting_web_flag, section_header_comment,
};
use crate::codegen::generator::wire::rust::internal_config::GeneratorWireRustInternalConfig;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFunc;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::WireRustOutputSpec;
use itertools::Itertools;
use strum::IntoEnumIterator;

// Call it "text", not "code", because the whole codegen is generating code,
// and we want to emphasize we are generating final output text here.
pub(super) struct WireRustOutputText {
    pub(super) text: String,
    pub(super) extern_funcs: Vec<ExternFunc>,
}

pub(super) fn generate(
    spec: &WireRustOutputSpec,
    config: &GeneratorWireRustInternalConfig,
) -> anyhow::Result<WireRustOutputText> {
    let merged_code_raw = generate_merged_code(spec);
    let merged_code = merged_code_raw.map(|code, _| WireRustOutputCode {
        body: code.body,
        extern_funcs: (code.extern_funcs.into_iter())
            .filter(|f| config.has_ffigen || !f.needs_ffigen)
            .collect(),
        extern_classes: (code.extern_classes.into_iter())
            .filter(|f| config.has_ffigen || !f.needs_ffigen)
            .collect(),
    });

    let text_acc = generate_text_from_merged_code(
        config,
        &(merged_code.clone()).map(|code, _| code.all_code(&config.c_symbol_prefix)),
    )?;
    let text = merge_rust_acc_into_one_file(text_acc);

    let extern_funcs = compute_extern_funcs(merged_code);

    Ok(WireRustOutputText { text, extern_funcs })
}

fn compute_extern_funcs(merged_code: Acc<WireRustOutputCode>) -> Vec<ExternFunc> {
    let extern_funcs_acc = merged_code.map(|code, _| code.extern_funcs);
    TargetOrCommon::iter()
        .flat_map(|target| extern_funcs_acc[target].clone())
        .collect_vec()
}

fn generate_merged_code(spec: &WireRustOutputSpec) -> Acc<WireRustOutputCode> {
    let mut merged_code = Acc::<Vec<WireRustOutputCode>>::default();
    let mut add = |section_name: &str, item: &Acc<Vec<WireRustOutputCode>>| {
        if !section_name.is_empty() {
            merged_code += section_header_comment(section_name, item);
        }
        merged_code += item.clone();
    };

    add("", &spec.misc.code_header);
    add("", &spec.misc.file_attributes);
    add("imports", &spec.misc.imports);
    add("boilerplate", &spec.misc.boilerplate);
    add("executor", &spec.misc.executor);
    add("wire_funcs", &spec.misc.wire_funcs);
    add("wrapper_structs", &spec.misc.wrapper_structs);
    add("static_checks", &spec.misc.static_checks);
    add("related_funcs", &spec.misc.related_funcs);
    add("extra_from_parser", &spec.misc.extra_from_parser);
    add("dart2rust", &spec.dart2rust.inner);
    add("rust2dart", &spec.rust2dart.inner);

    merged_code.map(|code, _| code.into_iter().fold(Default::default(), |a, b| a + b))
}

fn generate_text_from_merged_code(
    config: &GeneratorWireRustInternalConfig,
    core_code: &Acc<String>,
) -> anyhow::Result<Acc<Option<String>>> {
    Ok(generate_text_respecting_web_flag(
        core_code.clone(),
        config.web_enabled,
    ))
}

fn merge_rust_acc_into_one_file(acc: Acc<Option<String>>) -> String {
    let common = acc.common.unwrap_or_default();
    let io = (acc.io.as_ref())
        .map(|x| generate_inline_mod(x, Target::Io))
        .unwrap_or_default();
    let web = (acc.web.as_ref())
        .map(|x| generate_inline_mod(x, Target::Web))
        .unwrap_or_default();

    format!(
        "{common}
        {io}
        {web}"
    )
}

fn generate_inline_mod(mod_body: &str, target: Target) -> String {
    let name = target.to_string().to_lowercase();

    let prelude = match target {
        Target::Io => "",
        Target::Web => "/// cbindgen:ignore",
    };

    let cfg = match target {
        Target::Io => r#"not(target_family = "wasm")"#,
        Target::Web => r#"target_family = "wasm""#,
    };

    format!(
        "
        {prelude}
        #[cfg({cfg})]
        mod {name} {{
            {mod_body}
        }}
        #[cfg({cfg})]
        pub use {name}::*;
        "
    )
}
