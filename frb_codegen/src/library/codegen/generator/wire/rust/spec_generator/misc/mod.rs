use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::structs::CodecMode;
use crate::codegen::generator::misc::generate_code_header;
use crate::codegen::generator::misc::target::TargetOrCommon;
use crate::codegen::generator::wire::rust::spec_generator::base::{
    WireRustGenerator, WireRustGeneratorContext,
};
use crate::codegen::generator::wire::rust::spec_generator::misc::function::generate_wire_func;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::MirPackComputedCache;
use crate::codegen::ir::mir::func::MirFuncOwnerInfo;
use crate::codegen::ir::mir::pack::MirPack;
use crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode;
use crate::codegen::ir::mir::ty::MirType;
use crate::if_then_some;
use crate::library::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;
use crate::utils::namespace::Namespace;
use itertools::Itertools;
use serde::Serialize;
use sha1::{Digest, Sha1};
use std::collections::HashSet;

pub(crate) mod function;
pub(crate) mod ty;

#[derive(Serialize)]
pub(crate) struct WireRustOutputSpecMisc {
    pub code_header: Acc<Vec<WireRustOutputCode>>,
    pub file_attributes: Acc<Vec<WireRustOutputCode>>,
    pub imports: Acc<Vec<WireRustOutputCode>>,
    pub boilerplate: Acc<Vec<WireRustOutputCode>>,
    pub executor: Acc<Vec<WireRustOutputCode>>,
    pub wire_funcs: Acc<Vec<WireRustOutputCode>>,
    pub wrapper_structs: Acc<Vec<WireRustOutputCode>>,
    pub static_checks: Acc<Vec<WireRustOutputCode>>,
    pub related_funcs: Acc<Vec<WireRustOutputCode>>,
    pub extra_from_parser: Acc<Vec<WireRustOutputCode>>,
    pub content_hash: i32,
}

pub(crate) fn generate(
    context: WireRustGeneratorContext,
    cache: &MirPackComputedCache,
) -> anyhow::Result<WireRustOutputSpecMisc> {
    let content_hash = generate_content_hash(context.mir_pack);
    Ok(WireRustOutputSpecMisc {
        code_header: Acc::new(|_| vec![(generate_code_header() + "\n\n").into()]),
        file_attributes: Acc::new_common(vec![FILE_ATTRIBUTES.to_string().into()]),
        imports: generate_imports(&cache.distinct_types, context),
        executor: Acc::new_common(vec![generate_handler(context.mir_pack).into()]),
        boilerplate: generate_boilerplate(
            context.config.default_stream_sink_codec,
            context.config.default_rust_opaque_codec,
            content_hash,
            &context.config.rust_preamble,
        ),
        wire_funcs: (context.mir_pack.funcs_with_impl().iter())
            .map(|f| generate_wire_func(f, context))
            .collect(),
        wrapper_structs: Acc::default(),
        static_checks: Acc::new_common(vec![generate_static_checks(
            &cache.distinct_types,
            context,
        )
        .into()]),
        related_funcs: cache
            .distinct_types
            .iter()
            .map(|ty| WireRustGenerator::new(ty.clone(), context).generate_related_funcs())
            .collect(),
        extra_from_parser: Acc::new_common(vec![WireRustOutputCode {
            body: context.mir_pack.extra_rust_output_code.clone(),
            extern_funcs: vec![],
            extern_classes: vec![],
        }]),
        content_hash,
    })
}

const FILE_ATTRIBUTES: &str = r#"#![allow(
non_camel_case_types,
unused,
non_snake_case,
clippy::needless_return,
clippy::redundant_closure_call,
clippy::redundant_closure,
clippy::useless_conversion,
clippy::unit_arg,
clippy::unused_unit,
clippy::double_parens,
clippy::let_and_return,
clippy::too_many_arguments,
clippy::match_single_binding,
clippy::clone_on_copy,
clippy::let_unit_value,
clippy::deref_addrof,
clippy::explicit_auto_deref,
clippy::borrow_deref_ref,
clippy::needless_borrow
)]"#;

fn generate_imports(
    types: &[MirType],
    context: WireRustGeneratorContext,
) -> Acc<Vec<WireRustOutputCode>> {
    let output_namespace = Namespace::new_from_rust_crate_path(
        &context.config.rust_output_path,
        &context.config.rust_crate_dir,
    )
    .unwrap();
    let imports_from_types = types
        .iter()
        .flat_map(|ty| WireRustGenerator::new(ty.clone(), context).generate_imports())
        .flatten()
        .filter(|namespace| namespace != &output_namespace)
        .map(|namespace| format!("use {}::*;", namespace.joined_path))
        .collect::<HashSet<String>>()
        .into_iter()
        .join("\n");

    let imports_from_functions = (context.mir_pack.funcs_with_impl().iter())
        .filter_map(
            |func| if_then_some!(let MirFuncOwnerInfo::Method(method) = &func.owner, method),
        )
        .filter_map(|method| method.trait_def.clone().map(|x| x.name))
        .map(|name| format!("use {};\n", name.rust_style()))
        .unique()
        .join("");

    // NOTE Do *not* use imports when possible, instead use fully specified name directly
    let static_imports = "use flutter_rust_bridge::{Handler, IntoIntoDart};
use flutter_rust_bridge::for_generated::{Lockable, transform_result_dco, Lifetimeable};
use flutter_rust_bridge::for_generated::byteorder::{NativeEndian, WriteBytesExt, ReadBytesExt};";

    Acc::new(|target| {
        let platform_imports = match target {
            TargetOrCommon::Web => {
                "use super::*;
                use flutter_rust_bridge::for_generated::wasm_bindgen;
                use flutter_rust_bridge::for_generated::wasm_bindgen::prelude::*;\n"
            }
            TargetOrCommon::Io => "use super::*;",
            _ => "",
        };

        vec![(imports_from_types.clone()
            + &imports_from_functions
            + static_imports
            + platform_imports)
            .into()]
    })
}

fn generate_static_checks(types: &[MirType], context: WireRustGeneratorContext) -> String {
    let raw = types
        .iter()
        .filter_map(|ty| WireRustGenerator::new(ty.clone(), context).generate_static_checks())
        .collect_vec();

    if raw.is_empty() {
        return "".to_owned();
    }

    let mut lines = vec![];
    lines.push("#[allow(clippy::unnecessary_literal_unwrap)]".to_owned());
    lines.push("const _: fn() = || {".to_owned());
    lines.extend(raw);
    lines.push("};".to_owned());
    lines.join("\n")
}

fn generate_boilerplate(
    default_stream_sink_codec: CodecMode,
    default_rust_opaque_codec: RustOpaqueCodecMode,
    content_hash: i32,
    rust_preamble: &str,
) -> Acc<Vec<WireRustOutputCode>> {
    let rust_preamble_formatted = if rust_preamble.is_empty() {
        "".to_owned()
    } else {
        format!("{rust_preamble}\n\n")
    };

    Acc::new(|target| {
        match target {
            TargetOrCommon::Io | TargetOrCommon::Web => {
                vec![
                    // generate_boilerplate_frb_initialize_rust(target).into(),
                    // generate_boilerplate_dart_fn_deliver_output(target).into(),
                    format!(
                        "{rust_preamble_formatted}flutter_rust_bridge::frb_generated_boilerplate_{}!();",
                        target.to_string().to_lowercase()
                    )
                    .into(),
                ]
            }
            TargetOrCommon::Common => vec![format!(
                r#"{rust_preamble_formatted}flutter_rust_bridge::frb_generated_boilerplate!(
                    default_stream_sink_codec = {default_stream_sink_codec}Codec,
                    default_rust_opaque = RustOpaque{default_rust_opaque_codec},
                    default_rust_auto_opaque = RustAutoOpaque{default_rust_opaque_codec},
                );
                pub(crate) const FLUTTER_RUST_BRIDGE_CODEGEN_VERSION: &str = "{version}";
                pub(crate) const FLUTTER_RUST_BRIDGE_CODEGEN_CONTENT_HASH: i32 = {content_hash};
            "#,
                version = env!("CARGO_PKG_VERSION"),
            )
            .into()],
        }
    })
}

// fn generate_boilerplate_frb_initialize_rust(target: TargetOrCommon) -> ExternFunc {
//     let message_port_type = match target {
//         TargetOrCommon::Common | TargetOrCommon::Web => {
//             "flutter_rust_bridge::for_generated::MessagePort"
//         }
//         // to make cbingen/ffigen happy
//         TargetOrCommon::Io => "i64",
//     };
//
//     ExternFunc {
//         func_name: "frb_initialize_rust".into(),
//         params: vec![
//             ExternFuncParam {
//                 name: "dart_opaque_drop_port".to_owned(),
//                 rust_type: message_port_type.to_owned(),
//                 dart_type: "NativePortType".to_owned(),
//             },
//             ExternFuncParam {
//                 name: "dart_fn_invoke_port".to_owned(),
//                 rust_type: message_port_type.to_owned(),
//                 dart_type: "NativePortType".to_owned(),
//             },
//         ],
//         return_type: None,
//         body: format!(
//             "
//                 flutter_rust_bridge::for_generated::handler_initialize(
//                     &*{HANDLER_NAME},
//                     dart_opaque_drop_port,
//                     dart_fn_invoke_port,
//                 )
//                 "
//         ),
//         target: target.try_into().unwrap(),
//     }
// }

fn generate_handler(mir_pack: &MirPack) -> String {
    if let Some(existing_handler) = &mir_pack.existing_handler {
        format!("pub use {};", existing_handler.rust_style())
    } else {
        r#"flutter_rust_bridge::frb_generated_default_handler!();"#.to_owned()
    }
}

// TODO can compute hash for more things
fn generate_content_hash(mir_pack: &MirPack) -> i32 {
    let mut hasher = Sha1::new();
    hasher.update(
        (mir_pack.funcs_with_impl().iter())
            .map(|func| func.name.rust_style())
            .sorted()
            .join("\n")
            .as_bytes(),
    );
    let digest = hasher.finalize();
    i32::from_le_bytes(digest[..4].try_into().unwrap())
}
