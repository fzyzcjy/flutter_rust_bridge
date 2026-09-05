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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::codec::structs::CodecMode;
    use crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode;
    use std::path::PathBuf;

    fn config(web_enabled: bool) -> GeneratorWireRustInternalConfig {
        GeneratorWireRustInternalConfig {
            rust_crate_dir: PathBuf::new(),
            web_enabled,
            rust_output_path: PathBuf::new(),
            c_symbol_prefix: "frb_".into(),
            has_ffigen: false,
            default_stream_sink_codec: CodecMode::Sse,
            default_rust_opaque_codec: RustOpaqueCodecMode::Moi,
            rust_preamble: String::new(),
        }
    }

    /// Emits platform modules only for populated targets with their exact cfg attributes.
    #[test]
    fn merges_common_io_and_web_code_with_platform_specific_wrappers() {
        let text = merge_rust_acc_into_one_file(Acc {
            common: Some("common_code".into()),
            io: Some("io_code".into()),
            web: Some("web_code".into()),
        });

        assert!(text.contains("common_code"));
        assert!(text.contains("#[cfg(not(target_family = \"wasm\"))]"));
        assert!(text.contains("mod io {\n            io_code"));
        assert!(text.contains("pub use io::*;"));
        assert!(text.contains("/// cbindgen:ignore\n        #[cfg(target_family = \"wasm\")]"));
        assert!(text.contains("mod web {\n            web_code"));
        assert!(text.contains("pub use web::*;"));

        let common_only = merge_rust_acc_into_one_file(Acc {
            common: Some("shared".into()),
            io: None,
            web: None,
        });
        assert!(common_only.contains("shared"));
        assert!(!common_only.contains("mod io"));
        assert!(!common_only.contains("mod web"));
    }

    /// Keeps or removes web text according to the configured web flag.
    #[test]
    fn filters_web_text_while_preserving_common_and_io_text() {
        let core = Acc {
            common: "common".into(),
            io: "io".into(),
            web: "web".into(),
        };

        let enabled = generate_text_from_merged_code(&config(true), &core).unwrap();
        let disabled = generate_text_from_merged_code(&config(false), &core).unwrap();

        assert_eq!(enabled.common.as_deref(), Some("common"));
        assert_eq!(enabled.io.as_deref(), Some("io"));
        assert_eq!(enabled.web.as_deref(), Some("web"));
        assert_eq!(disabled.common.as_deref(), Some("common"));
        assert_eq!(disabled.io.as_deref(), Some("io"));
        assert_eq!(disabled.web, None);
    }
}
