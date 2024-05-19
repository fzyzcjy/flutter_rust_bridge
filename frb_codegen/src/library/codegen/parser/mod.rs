pub(crate) mod attribute_parser;
pub(crate) mod function_extractor;
pub(crate) mod function_parser;
pub(crate) mod internal_config;
pub(crate) mod misc;
pub(crate) mod reader;
pub(crate) mod source_graph;
pub(crate) mod type_alias_resolver;
pub(crate) mod type_parser;
mod unused_checker;

use crate::codegen::dumper::Dumper;
use crate::codegen::ir::namespace::{Namespace, NamespacedName};
use crate::codegen::ir::pack::IrPack;
use crate::codegen::misc::GeneratorProgressBarPack;
use crate::codegen::parser::function_extractor::{
    extract_generalized_functions_from_file, ExtractMode,
};
use crate::codegen::parser::function_parser::FunctionParser;
use crate::codegen::parser::internal_config::ParserInternalConfig;
use crate::codegen::parser::misc::parse_has_executor;
use crate::codegen::parser::reader::CachedRustReader;
use crate::codegen::parser::type_alias_resolver::resolve_type_aliases;
use crate::codegen::parser::type_parser::TypeParser;
use crate::codegen::parser::unused_checker::get_unused_types;
use crate::codegen::ConfigDumpContent;
use crate::library::misc::consts::HANDLER_NAME;
use anyhow::ensure;
use itertools::Itertools;
use log::trace;
use std::path::{Path, PathBuf};
use syn::File;
use ConfigDumpContent::SourceGraph;

pub(crate) fn parse(
    config: &ParserInternalConfig,
    cached_rust_reader: &mut CachedRustReader,
    dumper: &Dumper,
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<IrPack> {
    let rust_input_paths = &config.rust_input_path_pack.rust_input_paths;
    trace!("rust_input_paths={:?}", &rust_input_paths);

    let pb = progress_bar_pack.parse_source_graph.start();
    let crate_map = source_graph::crates::Crate::parse(
        &config.rust_crate_dir.join("Cargo.toml"),
        cached_rust_reader,
        dumper,
    )?;
    dumper.dump(SourceGraph, "source_graph.json", &crate_map)?;
    drop(pb);

    let crate_all_rust_paths = get_crate_all_rust_paths(&config.rust_crate_dir)?;
    let all_file_data_arr = read_files(
        crate_all_rust_paths,
        &config.rust_crate_dir,
        cached_rust_reader,
        dumper,
        progress_bar_pack,
    )?;

    let src_fns = all_file_data_arr
        .iter()
        .map(|file| {
            let mode = if rust_input_paths.contains(&file.path) {
                ExtractMode::Full
            } else {
                ExtractMode::MethodOnly
            };
            extract_generalized_functions_from_file(&file.ast, &file.path, mode)
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect_vec();

    let src_structs = crate_map.root_module().collect_structs();
    let src_enums = crate_map.root_module().collect_enums();
    let src_types = resolve_type_aliases(crate_map.root_module().collect_types());

    let mut type_parser = TypeParser::new(src_structs.clone(), src_enums.clone(), src_types);
    let mut function_parser = FunctionParser::new(&mut type_parser);

    let ir_funcs = src_fns
        .iter()
        .enumerate()
        .map(|(index, f)| {
            function_parser.parse_function(
                &f.generalized_item_fn,
                &f.path,
                &config.rust_crate_dir,
                &config.force_codec_mode_pack,
                (index + 1) as i32,
                config.default_stream_sink_codec,
                config.default_rust_opaque_codec,
            )
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        // to give downstream a stable output
        .sorted_by_cached_key(|func| func.name.clone())
        .collect_vec();

    let existing_handlers = (all_file_data_arr.iter())
        .filter(|file| rust_input_paths.contains(&file.path))
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

    let (struct_pool, enum_pool, dart_code_of_type) = type_parser.consume();

    let mut ans = IrPack {
        funcs: ir_funcs,
        struct_pool,
        enum_pool,
        dart_code_of_type,
        existing_handler: existing_handlers.first().cloned(),
        unused_types: vec![],
    };

    ans.unused_types = get_unused_types(
        &ans,
        &src_structs,
        &src_enums,
        &config.rust_input_path_pack,
        &config.rust_crate_dir,
    )?;

    Ok(ans)
}

struct FileData {
    path: PathBuf,
    content: String,
    ast: File,
}

fn read_files(
    paths: &[PathBuf],
    rust_crate_dir: &Path,
    cached_rust_reader: &mut CachedRustReader,
    dumper: &Dumper,
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<Vec<FileData>> {
    let _pb = progress_bar_pack.parse_cargo_expand.start();
    let contents = paths
        .iter()
        .map(|path| {
            let content = cached_rust_reader.read_rust_file(path, rust_crate_dir, dumper)?;
            Ok((path.to_owned(), content))
        })
        .collect::<anyhow::Result<Vec<(PathBuf, String)>>>()?;

    contents
        .into_iter()
        .map(|(path, content)| {
            let ast = syn::parse_file(&content)?;
            Ok(FileData { path, content, ast })
        })
        .collect()
}

fn get_crate_all_rust_paths(rust_crate_dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    TODO
}

#[cfg(test)]
mod tests {
    use crate::codegen::config::internal_config::RustInputPathPack;
    use crate::codegen::config::internal_config_parser::compute_force_codec_mode_pack;
    use crate::codegen::dumper::Dumper;
    use crate::codegen::generator::codec::structs::CodecMode;
    use crate::codegen::ir::ty::rust_opaque::RustOpaqueCodecMode;
    use crate::codegen::misc::GeneratorProgressBarPack;
    use crate::codegen::parser::internal_config::ParserInternalConfig;
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
            Some(Box::new(|rust_crate_dir| RustInputPathPack {
                rust_input_paths: [
                    rust_crate_dir.join("src/api_one.rs"),
                    rust_crate_dir.join("src/api_two.rs"),
                ]
                .into(),
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
        rust_input_path_pack: Option<Box<dyn Fn(&Path) -> RustInputPathPack>>,
    ) -> anyhow::Result<()> {
        let (actual_ir, rust_crate_dir) = execute_parse(fixture_name, rust_input_path_pack)?;
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
        rust_input_path_pack: Option<Box<dyn Fn(&Path) -> RustInputPathPack>>,
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
                rust_input_path_pack: rust_input_path_pack.map(|f| f(&rust_crate_dir)).unwrap_or(
                    RustInputPathPack {
                        rust_input_paths: vec![rust_crate_dir.join("src/api.rs")],
                    },
                ),
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
