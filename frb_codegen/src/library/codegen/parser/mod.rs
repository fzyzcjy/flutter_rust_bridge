pub(crate) mod attribute_parser;
mod auto_accessor_parser;
mod file_reader;
pub(crate) mod function_extractor;
pub(crate) mod function_parser;
pub(crate) mod internal_config;
pub(crate) mod misc;
pub(crate) mod reader;
pub(crate) mod sanity_checker;
pub(crate) mod source_graph;
pub(crate) mod type_alias_resolver;
pub(crate) mod type_parser;

use crate::codegen::dumper::Dumper;
use crate::codegen::ir::func::IrFunc;
use crate::codegen::ir::namespace::{Namespace, NamespacedName};
use crate::codegen::ir::pack::IrPack;
use crate::codegen::misc::GeneratorProgressBarPack;
use crate::codegen::parser::auto_accessor_parser::parse_auto_accessors;
use crate::codegen::parser::file_reader::{read_files, FileData};
use crate::codegen::parser::function_extractor::extract_generalized_functions_from_file;
use crate::codegen::parser::function_extractor::structs::PathAndItemFn;
use crate::codegen::parser::function_parser::FunctionParser;
use crate::codegen::parser::internal_config::ParserInternalConfig;
use crate::codegen::parser::misc::parse_has_executor;
use crate::codegen::parser::reader::CachedRustReader;
use crate::codegen::parser::sanity_checker::misc_checker::check_suppressed_input_path_no_content;
use crate::codegen::parser::sanity_checker::opaque_inside_translatable_checker::check_opaque_inside_translatable;
use crate::codegen::parser::sanity_checker::unused_checker::get_unused_types;
use crate::codegen::parser::source_graph::modules::Struct;
use crate::codegen::parser::type_alias_resolver::resolve_type_aliases;
use crate::codegen::parser::type_parser::TypeParser;
use crate::codegen::ConfigDumpContent;
use crate::library::misc::consts::HANDLER_NAME;
use anyhow::ensure;
use itertools::{concat, Itertools};
use log::trace;
use std::collections::HashMap;
use std::path::Path;
use syn::Visibility;
use ConfigDumpContent::SourceGraph;

pub(crate) fn parse(
    config: &ParserInternalConfig,
    cached_rust_reader: &mut CachedRustReader,
    dumper: &Dumper,
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<IrPack> {
    check_suppressed_input_path_no_content(
        &config
            .rust_input_namespace_pack
            .rust_suppressed_input_namespaces,
        &config.rust_crate_dir,
        cached_rust_reader,
        dumper,
    )?;

    let rust_input_paths = &config.rust_input_namespace_pack.rust_input_namespaces;
    trace!("rust_input_paths={:?}", &rust_input_paths);

    let pb = progress_bar_pack.parse_cargo_expand.start();
    let file_data_arr = read_files(
        rust_input_paths,
        &config.rust_crate_dir,
        cached_rust_reader,
        dumper,
    )?;
    drop(pb);

    let pb = progress_bar_pack.parse_source_graph.start();
    let crate_map = source_graph::crates::Crate::parse(
        &config.rust_crate_dir.join("Cargo.toml"),
        cached_rust_reader,
        dumper,
    )?;
    dumper.dump(SourceGraph, "source_graph.json", &crate_map)?;
    drop(pb);

    let src_fns_all = file_data_arr
        .iter()
        .map(|file| extract_generalized_functions_from_file(&file.ast, &file.path))
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect_vec();
    let (src_fns_interest, src_fns_skipped): (Vec<_>, Vec<_>) = (src_fns_all.into_iter())
        .partition(|item| matches!(item.generalized_item_fn.vis(), Visibility::Public(_)));

    let src_structs = crate_map.root_module().collect_structs();
    let src_enums = crate_map.root_module().collect_enums();
    let src_types = resolve_type_aliases(crate_map.root_module().collect_types());

    let mut type_parser = TypeParser::new(src_structs.clone(), src_enums.clone(), src_types);

    let ir_funcs = parse_ir_funcs(config, &src_fns_interest, &mut type_parser, &src_structs)?;

    let existing_handlers = parse_existing_handlers(config, &file_data_arr)?;

    let (struct_pool, enum_pool, dart_code_of_type) = type_parser.consume();

    let mut ans = IrPack {
        funcs: ir_funcs,
        struct_pool,
        enum_pool,
        dart_code_of_type,
        existing_handler: existing_handlers.first().cloned(),
        unused_types: vec![],
        skipped_functions: compute_skipped_functions(&src_fns_skipped, &config.rust_crate_dir)?,
    };

    ans.unused_types = get_unused_types(
        &ans,
        &src_structs,
        &src_enums,
        &config.rust_input_namespace_pack.rust_input_namespaces,
        &config.rust_crate_dir,
    )?;

    check_opaque_inside_translatable(&ans);

    Ok(ans)
}

fn parse_ir_funcs(
    config: &ParserInternalConfig,
    src_fns: &[PathAndItemFn],
    type_parser: &mut TypeParser,
    src_structs: &HashMap<String, &Struct>,
) -> anyhow::Result<Vec<IrFunc>> {
    let mut function_parser = FunctionParser::new(type_parser);

    let ir_funcs_normal = src_fns
        .iter()
        .map(|f| {
            function_parser.parse_function(
                &f.generalized_item_fn,
                &f.path,
                &config.rust_crate_dir,
                &config.force_codec_mode_pack,
                config.default_stream_sink_codec,
                config.default_rust_opaque_codec,
            )
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect_vec();

    let ir_funcs_auto_accessor = parse_auto_accessors(config, src_structs, type_parser)?;

    Ok(concat([ir_funcs_normal, ir_funcs_auto_accessor])
        .into_iter()
        // to give downstream a stable output
        .sorted_by_cached_key(|func| func.name.clone())
        .enumerate()
        .map(|(index, f)| IrFunc {
            id: Some((index + 1) as _),
            ..f
        })
        .collect_vec())
}

fn parse_existing_handlers(
    config: &ParserInternalConfig,
    file_data_arr: &[FileData],
) -> anyhow::Result<Vec<NamespacedName>> {
    let existing_handlers = (file_data_arr.iter())
        .filter(|file| parse_has_executor(&file.content))
        .map(|file| {
            NamespacedName::new(
                Namespace::new_from_rust_crate_path(&file.path, &config.rust_crate_dir).unwrap(),
                HANDLER_NAME.to_owned(),
            )
        })
        .collect_vec();
    ensure!(
        existing_handlers.len() <= 1,
        // frb-coverage:ignore-start
        // This will stop the whole generator and tell the users, so we do not care about testing it
        "Should have at most one custom handler"
    );
    // frb-coverage:ignore-end
    Ok(existing_handlers)
}

fn compute_skipped_functions(
    src_fns_skipped: &[PathAndItemFn],
    rust_crate_dir: &Path,
) -> anyhow::Result<Vec<NamespacedName>> {
    src_fns_skipped
        .iter()
        .map(|x| {
            Ok(NamespacedName::new(
                Namespace::new_from_rust_crate_path(&x.path, rust_crate_dir)?,
                x.generalized_item_fn.sig().ident.to_string(),
            ))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::codegen::config::internal_config_parser::compute_force_codec_mode_pack;
    use crate::codegen::dumper::Dumper;
    use crate::codegen::generator::codec::structs::CodecMode;
    use crate::codegen::ir::ty::rust_opaque::RustOpaqueCodecMode;
    use crate::codegen::misc::GeneratorProgressBarPack;
    use crate::codegen::parser::internal_config::ParserInternalConfig;
    use crate::codegen::parser::internal_config::RustInputNamespacePack;
    use crate::codegen::parser::parse;
    use crate::codegen::parser::reader::CachedRustReader;
    use crate::codegen::parser::source_graph::crates::Crate;
    use crate::codegen::parser::IrPack;
    use crate::utils::logs::configure_opinionated_test_logging;
    use crate::utils::test_utils::{
        create_path_sanitizers, get_test_fixture_dir, json_golden_test,
    };
    use log::info;
    use serial_test::serial;
    use std::path::{Path, PathBuf};

    #[test]
    #[serial]
    fn test_simple() -> anyhow::Result<()> {
        body("library/codegen/parser/mod/simple", None)
    }

    #[test]
    #[serial]
    fn test_methods() -> anyhow::Result<()> {
        body("library/codegen/parser/mod/methods", None)
    }

    #[test]
    #[serial]
    fn test_multi_input_file() -> anyhow::Result<()> {
        body(
            "library/codegen/parser/mod/multi_input_file",
            Some(Box::new(|rust_crate_dir| RustInputNamespacePack {
                rust_input_namespaces: [
                    rust_crate_dir.join("src/api_one.rs"),
                    rust_crate_dir.join("src/api_two.rs"),
                ]
                .into(),
                rust_suppressed_input_namespaces: vec![],
            })),
        )
    }

    #[test]
    #[serial]
    fn test_use_type_in_another_file() -> anyhow::Result<()> {
        body("library/codegen/parser/mod/use_type_in_another_file", None)
    }

    #[test]
    #[serial]
    fn test_qualified_names() -> anyhow::Result<()> {
        body("library/codegen/parser/mod/qualified_names", None)
    }

    #[test]
    #[serial]
    fn test_non_qualified_names() -> anyhow::Result<()> {
        body("library/codegen/parser/mod/non_qualified_names", None)
    }

    #[test]
    #[serial]
    fn test_generics() -> anyhow::Result<()> {
        body("library/codegen/parser/mod/generics", None)
    }

    #[test]
    #[serial]
    fn test_unused_struct_enum() -> anyhow::Result<()> {
        body("library/codegen/parser/mod/unused_struct_enum", None)
    }

    #[allow(clippy::type_complexity)]
    fn body(
        fixture_name: &str,
        rust_input_namespace_pack: Option<Box<dyn Fn(&Path) -> RustInputNamespacePack>>,
    ) -> anyhow::Result<()> {
        let (actual_ir, rust_crate_dir) = execute_parse(fixture_name, rust_input_namespace_pack)?;
        json_golden_test(
            &serde_json::to_value(actual_ir)?,
            &rust_crate_dir.join("expect_ir.json"),
            &[],
        )?;

        Ok(())
    }

    #[allow(clippy::type_complexity)]
    fn execute_parse(
        fixture_name: &str,
        rust_input_namespace_pack: Option<Box<dyn Fn(&Path) -> RustInputNamespacePack>>,
    ) -> anyhow::Result<(IrPack, PathBuf)> {
        configure_opinionated_test_logging();
        let test_fixture_dir = get_test_fixture_dir(fixture_name);
        let rust_crate_dir = test_fixture_dir.clone();
        info!("test_fixture_dir={test_fixture_dir:?}");

        let crate_map = Crate::parse(
            &rust_crate_dir.join("Cargo.toml"),
            &mut CachedRustReader::default(),
            &Dumper(&Default::default()),
        )?;
        json_golden_test(
            &serde_json::to_value(crate_map)?,
            &rust_crate_dir.join("expect_source_graph.json"),
            &create_path_sanitizers(&test_fixture_dir),
        )
        .unwrap();

        let pack = parse(
            &ParserInternalConfig {
                rust_input_namespace_pack: rust_input_namespace_pack
                    .map(|f| f(&rust_crate_dir))
                    .unwrap_or(RustInputNamespacePack {
                        rust_input_namespaces: vec![rust_crate_dir.join("src/api.rs")],
                        rust_suppressed_input_namespaces: vec![],
                    }),
                rust_crate_dir: rust_crate_dir.clone(),
                force_codec_mode_pack: compute_force_codec_mode_pack(true),
                default_stream_sink_codec: CodecMode::Dco,
                default_rust_opaque_codec: RustOpaqueCodecMode::Nom,
            },
            &mut CachedRustReader::default(),
            &Dumper(&Default::default()),
            &GeneratorProgressBarPack::new(),
        )?;

        Ok((pack, rust_crate_dir))
    }
}
