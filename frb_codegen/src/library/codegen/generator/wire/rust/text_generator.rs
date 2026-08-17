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

    let extern_funcs = compute_extern_funcs(merged_code.clone());
    let text_acc = generate_text_from_merged_code(
        config,
        &(merged_code.clone()).map(|code, _| code.all_code(&config.c_symbol_prefix)),
    )?;
    let text = merge_rust_acc_into_one_file(text_acc)
        + &generate_static_link_anchor(config, &extern_funcs);

    Ok(WireRustOutputText { text, extern_funcs })
}

fn generate_static_link_anchor(
    config: &GeneratorWireRustInternalConfig,
    extern_funcs: &[ExternFunc],
) -> String {
    let referenced_functions = [
        "frb_get_rust_content_hash".to_owned(),
        "frb_pde_ffi_dispatcher_primary".to_owned(),
        "frb_pde_ffi_dispatcher_sync".to_owned(),
        "frb_dart_fn_deliver_output".to_owned(),
        "frb_link_store_dart_post_cobject".to_owned(),
        "frb_link_dart_opaque_dart2rust_encode".to_owned(),
        "frb_link_dart_opaque_drop_thread_box_persistent_handle".to_owned(),
        "frb_link_dart_opaque_rust2dart_decode".to_owned(),
        "frb_link_rust_vec_u8_new".to_owned(),
        "frb_link_rust_vec_u8_resize".to_owned(),
        "frb_link_rust_vec_u8_free".to_owned(),
        "frb_link_init_frb_dart_api_dl".to_owned(),
        "frb_link_free_wire_sync_rust2dart_dco".to_owned(),
        "frb_link_free_wire_sync_rust2dart_sse".to_owned(),
        "frb_link_create_shutdown_callback".to_owned(),
    ]
    .into_iter()
    .chain(
        extern_funcs
            .iter()
            .filter(|func| func.target == Target::Io)
            .map(|func| func.func_name(&config.c_symbol_prefix)),
    )
    .map(|name| format!("{name} as *const () as usize"))
    .join(",\n");

    format!(
        r#"
        #[cfg(not(target_family = "wasm"))]
        unsafe extern "C" {{
            #[link_name = "store_dart_post_cobject"]
            fn frb_link_store_dart_post_cobject();
            #[link_name = "frb_dart_opaque_dart2rust_encode"]
            fn frb_link_dart_opaque_dart2rust_encode();
            #[link_name = "frb_dart_opaque_drop_thread_box_persistent_handle"]
            fn frb_link_dart_opaque_drop_thread_box_persistent_handle();
            #[link_name = "frb_dart_opaque_rust2dart_decode"]
            fn frb_link_dart_opaque_rust2dart_decode();
            #[link_name = "frb_rust_vec_u8_new"]
            fn frb_link_rust_vec_u8_new();
            #[link_name = "frb_rust_vec_u8_resize"]
            fn frb_link_rust_vec_u8_resize();
            #[link_name = "frb_rust_vec_u8_free"]
            fn frb_link_rust_vec_u8_free();
            #[link_name = "frb_init_frb_dart_api_dl"]
            fn frb_link_init_frb_dart_api_dl();
            #[link_name = "frb_free_wire_sync_rust2dart_dco"]
            fn frb_link_free_wire_sync_rust2dart_dco();
            #[link_name = "frb_free_wire_sync_rust2dart_sse"]
            fn frb_link_free_wire_sync_rust2dart_sse();
            #[link_name = "frb_create_shutdown_callback"]
            fn frb_link_create_shutdown_callback();
        }}

        #[cfg(not(target_family = "wasm"))]
        #[unsafe(export_name = "{prefix}link_anchor")]
        pub extern "C" fn frb_link_anchor() {{
            std::hint::black_box([
                {referenced_functions}
            ]);
        }}
        "#,
        prefix = config.c_symbol_prefix,
    )
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
