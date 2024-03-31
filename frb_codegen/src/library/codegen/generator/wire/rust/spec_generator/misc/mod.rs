use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::structs::CodecMode;
use crate::codegen::generator::misc::generate_code_header;
use crate::codegen::generator::misc::target::TargetOrCommon;
use crate::codegen::generator::wire::rust::spec_generator::base::{
    WireRustGenerator, WireRustGeneratorContext,
};
use crate::codegen::generator::wire::rust::spec_generator::misc::wire_func::generate_wire_func;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::IrPackComputedCache;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::rust_opaque::RustOpaqueCodecMode;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;
use itertools::Itertools;
use serde::Serialize;
use std::collections::HashSet;

pub(crate) mod ty;
pub(crate) mod wire_func;

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
}

pub(crate) fn generate(
    context: WireRustGeneratorContext,
    cache: &IrPackComputedCache,
) -> anyhow::Result<WireRustOutputSpecMisc> {
    Ok(WireRustOutputSpecMisc {
        code_header: Acc::new(|_| vec![(generate_code_header() + "\n\n").into()]),
        file_attributes: Acc::new_common(vec![FILE_ATTRIBUTES.to_string().into()]),
        imports: generate_imports(&cache.distinct_types, context),
        executor: Acc::new_common(vec![generate_executor(context.ir_pack).into()]),
        boilerplate: generate_boilerplate(
            context.config.default_stream_sink_codec,
            context.config.default_rust_opaque_codec,
        ),
        wire_funcs: (context.ir_pack.funcs.iter())
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
clippy::match_single_binding
)]"#;

fn generate_imports(
    types: &[IrType],
    context: WireRustGeneratorContext,
) -> Acc<Vec<WireRustOutputCode>> {
    let imports_from_types = types
        .iter()
        .flat_map(|ty| WireRustGenerator::new(ty.clone(), context).generate_imports())
        .flatten()
        .collect::<HashSet<String>>()
        .into_iter()
        .join("\n");

    // NOTE Do *not* use imports when possible, instead use fully specified name directly
    let static_imports = "use flutter_rust_bridge::{Handler, IntoIntoDart};
use flutter_rust_bridge::for_generated::transform_result_dco;
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

        vec![(imports_from_types.clone() + static_imports + platform_imports).into()]
    })
}

fn generate_static_checks(types: &[IrType], context: WireRustGeneratorContext) -> String {
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
) -> Acc<Vec<WireRustOutputCode>> {
    Acc::new(|target| {
        match target {
            TargetOrCommon::Io | TargetOrCommon::Web => {
                vec![
                    // generate_boilerplate_frb_initialize_rust(target).into(),
                    // generate_boilerplate_dart_fn_deliver_output(target).into(),
                    format!(
                        "flutter_rust_bridge::frb_generated_boilerplate_{}!();",
                        target.to_string().to_lowercase()
                    )
                    .into(),
                ]
            }
            TargetOrCommon::Common => vec![format!(
                r#"
                flutter_rust_bridge::frb_generated_boilerplate!(
                    default_stream_sink_codec = {default_stream_sink_codec}Codec,
                    default_rust_opaque = RustOpaque{default_rust_opaque_codec},
                    default_rust_auto_opaque = RustAutoOpaque{default_rust_opaque_codec},
                );
                const FLUTTER_RUST_BRIDGE_CODEGEN_VERSION: &str = "{version}";
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

fn generate_executor(ir_pack: &IrPack) -> String {
    if ir_pack.has_executor {
        "/* nothing since executor detected */".to_owned()
    } else {
        r#"flutter_rust_bridge::frb_generated_default_handler!();"#.to_owned()
    }
}
