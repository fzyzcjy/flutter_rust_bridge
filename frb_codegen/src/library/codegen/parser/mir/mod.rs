pub(crate) mod attribute_parser;
mod auto_accessor_parser;
mod existing_handler;
mod file_reader;
pub(crate) mod function_parser;
pub(crate) mod internal_config;
pub(crate) mod misc;
pub(crate) mod reader;
pub(crate) mod sanity_checker;
pub(crate) mod type_parser;

use crate::codegen::dumper::Dumper;
use crate::codegen::ir::hir::flat::HirFlatCrate;
use crate::codegen::ir::hir::hierarchical::crates::HirCrate;
use crate::codegen::ir::hir::hierarchical::function::HirFunction;
use crate::codegen::ir::hir::hierarchical::struct_or_enum::HirStruct;
use crate::codegen::ir::mir::func::MirFunc;
use crate::codegen::ir::mir::namespace::{Namespace, NamespacedName};
use crate::codegen::ir::mir::pack::MirPack;
use crate::codegen::mir_parser::auto_accessor_parser::parse_auto_accessors;
use crate::codegen::mir_parser::function_parser::FunctionParser;
use crate::codegen::mir_parser::internal_config::ParserInternalConfig;
use crate::codegen::mir_parser::reader::CachedRustReader;
use crate::codegen::mir_parser::sanity_checker::opaque_inside_translatable_checker::check_opaque_inside_translatable;
use crate::codegen::mir_parser::sanity_checker::unused_checker::get_unused_types;
use crate::codegen::mir_parser::type_parser::TypeParser;
use crate::codegen::misc::GeneratorProgressBarPack;
use crate::codegen::ConfigDumpContent;
use crate::library::misc::consts::HANDLER_NAME;
use anyhow::ensure;
use itertools::{concat, Itertools};
use log::trace;
use std::collections::HashMap;
use std::path::Path;
use syn::Visibility;

pub(crate) fn parse(
    config: &ParserInternalConfig,
    hir_flat_crate: &HirFlatCrate,
) -> anyhow::Result<MirPack> {
    let (src_fns_interest, src_fns_skipped): (Vec<_>, Vec<_>) = (hir_flat_crate.functions.iter())
        .partition(|item| matches!(item.inner.vis(), Visibility::Public(_)));

    let mut type_parser = TypeParser::new(
        hir_flat_crate.structs.clone(),
        hir_flat_crate.enums.clone(),
        hir_flat_crate.types.clone(),
    );

    let mir_funcs = parse_mir_funcs(
        config,
        &src_fns_interest,
        &mut type_parser,
        &hir_flat_crate.structs,
    )?;

    let existing_handlers =
        existing_handler::parse_existing_handlers(config, &hir_flat_crate.modules)?;

    let (struct_pool, enum_pool, dart_code_of_type) = type_parser.consume();

    let mut ans = MirPack {
        funcs: mir_funcs,
        struct_pool,
        enum_pool,
        dart_code_of_type,
        existing_handler: existing_handlers.first().cloned(),
        unused_types: vec![],
        skipped_functions: compute_skipped_functions(&src_fns_skipped)?,
    };

    ans.unused_types = get_unused_types(
        &ans,
        &hir_flat_crate.structs,
        &hir_flat_crate.enums,
        &config.rust_input_namespace_pack,
    )?;

    check_opaque_inside_translatable(&ans);

    Ok(ans)
}

fn parse_mir_funcs(
    config: &ParserInternalConfig,
    src_fns: &[&HirFunction],
    type_parser: &mut TypeParser,
    src_structs: &HashMap<String, &HirStruct>,
) -> anyhow::Result<Vec<MirFunc>> {
    let mut function_parser = FunctionParser::new(type_parser);

    let mir_funcs_normal = src_fns
        .iter()
        .map(|f| {
            function_parser.parse_function(
                &f.inner,
                &f.namespace,
                &config.force_codec_mode_pack,
                config.default_stream_sink_codec,
                config.default_rust_opaque_codec,
            )
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect_vec();

    let mir_funcs_auto_accessor = parse_auto_accessors(config, src_structs, type_parser)?;

    Ok(concat([mir_funcs_normal, mir_funcs_auto_accessor])
        .into_iter()
        // to give downstream a stable output
        .sorted_by_cached_key(|func| func.name.clone())
        .enumerate()
        .map(|(index, f)| MirFunc {
            id: Some((index + 1) as _),
            ..f
        })
        .collect_vec())
}

fn compute_skipped_functions(
    src_fns_skipped: &[&HirFunction],
) -> anyhow::Result<Vec<NamespacedName>> {
    src_fns_skipped
        .iter()
        .map(|x| {
            Ok(NamespacedName::new(
                x.namespace.to_owned(),
                x.inner.sig().ident.to_string(),
            ))
        })
        .collect()
}

// TODO
// #[cfg(test)]
// mod tests {
//     use crate::codegen::config::internal_config_parser::compute_force_codec_mode_pack;
//     use crate::codegen::dumper::Dumper;
//     use crate::codegen::generator::codec::structs::CodecMode;
//     use crate::codegen::ir::hir::hierarchical::crates::HirCrate;
//     use crate::codegen::ir::mir::namespace::Namespace;
//     use crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode;
//     use crate::codegen::misc::GeneratorProgressBarPack;
//     use crate::codegen::parser::internal_config::ParserInternalConfig;
//     use crate::codegen::parser::internal_config::RustInputNamespacePack;
//     use crate::codegen::parser::parse;
//     use crate::codegen::parser::reader::CachedRustReader;
//     use crate::codegen::parser::MirPack;
//     use crate::utils::logs::configure_opinionated_test_logging;
//     use crate::utils::test_utils::{
//         create_path_sanitizers, get_test_fixture_dir, json_golden_test,
//     };
//     use log::info;
//     use serial_test::serial;
//     use std::path::{Path, PathBuf};
//
//     #[test]
//     #[serial]
//     fn test_simple() -> anyhow::Result<()> {
//         body("library/codegen/parser/mod/simple", None)
//     }
//
//     #[test]
//     #[serial]
//     fn test_methods() -> anyhow::Result<()> {
//         body("library/codegen/parser/mod/methods", None)
//     }
//
//     #[test]
//     #[serial]
//     fn test_multi_input_file() -> anyhow::Result<()> {
//         body(
//             "library/codegen/parser/mod/multi_input_file",
//             Some(Box::new(|rust_crate_dir| RustInputNamespacePack {
//                 rust_input_namespaces: [
//                     Namespace::new_self_crate("api_one".to_owned()),
//                     Namespace::new_self_crate("api_two".to_owned()),
//                 ]
//                 .into(),
//             })),
//         )
//     }
//
//     #[test]
//     #[serial]
//     fn test_use_type_in_another_file() -> anyhow::Result<()> {
//         body("library/codegen/parser/mod/use_type_in_another_file", None)
//     }
//
//     #[test]
//     #[serial]
//     fn test_qualified_names() -> anyhow::Result<()> {
//         body("library/codegen/parser/mod/qualified_names", None)
//     }
//
//     #[test]
//     #[serial]
//     fn test_non_qualified_names() -> anyhow::Result<()> {
//         body("library/codegen/parser/mod/non_qualified_names", None)
//     }
//
//     #[test]
//     #[serial]
//     fn test_generics() -> anyhow::Result<()> {
//         body("library/codegen/parser/mod/generics", None)
//     }
//
//     #[test]
//     #[serial]
//     fn test_unused_struct_enum() -> anyhow::Result<()> {
//         body("library/codegen/parser/mod/unused_struct_enum", None)
//     }
//
//     #[allow(clippy::type_complexity)]
//     fn body(
//         fixture_name: &str,
//         rust_input_namespace_pack: Option<Box<dyn Fn(&Path) -> RustInputNamespacePack>>,
//     ) -> anyhow::Result<()> {
//         let (actual_ir, rust_crate_dir) = execute_parse(fixture_name, rust_input_namespace_pack)?;
//         json_golden_test(
//             &serde_json::to_value(actual_ir)?,
//             &rust_crate_dir.join("expect_ir.json"),
//             &[],
//         )?;
//
//         Ok(())
//     }
//
//     #[allow(clippy::type_complexity)]
//     fn execute_parse(
//         fixture_name: &str,
//         rust_input_namespace_pack: Option<Box<dyn Fn(&Path) -> RustInputNamespacePack>>,
//     ) -> anyhow::Result<(MirPack, PathBuf)> {
//         configure_opinionated_test_logging();
//         let test_fixture_dir = get_test_fixture_dir(fixture_name);
//         let rust_crate_dir = test_fixture_dir.clone();
//         info!("test_fixture_dir={test_fixture_dir:?}");
//
//         let crate_map = HirCrate::parse(
//             &rust_crate_dir.join("Cargo.toml"),
//             &mut CachedRustReader::default(),
//             &Dumper(&Default::default()),
//         )?;
//         json_golden_test(
//             &serde_json::to_value(crate_map)?,
//             &rust_crate_dir.join("expect_source_graph.json"),
//             &create_path_sanitizers(&test_fixture_dir),
//         )
//         .unwrap();
//
//         let pack = parse(
//             &ParserInternalConfig {
//                 rust_input_namespace_pack: rust_input_namespace_pack
//                     .map(|f| f(&rust_crate_dir))
//                     .unwrap_or(RustInputNamespacePack {
//                         rust_input_namespaces: vec![Namespace::new_self_crate("api".to_owned())],
//                     }),
//                 rust_crate_dir: rust_crate_dir.clone(),
//                 force_codec_mode_pack: compute_force_codec_mode_pack(true),
//                 default_stream_sink_codec: CodecMode::Dco,
//                 default_rust_opaque_codec: RustOpaqueCodecMode::Nom,
//             },
//             &mut CachedRustReader::default(),
//             &Dumper(&Default::default()),
//             &GeneratorProgressBarPack::new(),
//         )?;
//
//         Ok((pack, rust_crate_dir))
//     }
// }
