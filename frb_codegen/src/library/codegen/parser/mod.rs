pub(crate) mod attribute_parser;
pub(crate) mod function_extractor;
pub(crate) mod function_parser;
pub(crate) mod internal_config;
pub(crate) mod misc;
pub(crate) mod reader;
pub(crate) mod source_graph;
pub(crate) mod type_alias_resolver;
pub(crate) mod type_parser;

use crate::codegen::dumper::Dumper;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::parser::function_extractor::extract_generalized_functions_from_file;
use crate::codegen::parser::function_parser::FunctionParser;
use crate::codegen::parser::internal_config::ParserInternalConfig;
use crate::codegen::parser::misc::parse_has_executor;
use crate::codegen::parser::reader::CachedRustReader;
use crate::codegen::parser::type_alias_resolver::resolve_type_aliases;
use crate::codegen::parser::type_parser::TypeParser;
use crate::codegen::ConfigDumpContent;
use crate::utils::console::simple_progress;
use itertools::Itertools;
use log::trace;
use std::path::{Path, PathBuf};
use syn::File;
use ConfigDumpContent::SourceGraph;

pub(crate) fn parse(
    config: &ParserInternalConfig,
    cached_rust_reader: &mut CachedRustReader,
    dumper: &Dumper,
) -> anyhow::Result<IrPack> {
    let rust_input_paths = &config.rust_input_path_pack.rust_input_paths;
    trace!("rust_input_paths={:?}", &rust_input_paths);

    let file_data_arr = read_files(
        rust_input_paths,
        &config.rust_crate_dir,
        cached_rust_reader,
        dumper,
    )?;

    let pb = simple_progress("Parse crate source graph".to_owned(), 1);
    let crate_map = source_graph::crates::Crate::parse(
        &config.rust_crate_dir.join("Cargo.toml"),
        cached_rust_reader,
        dumper,
    )?;
    dumper.dump(SourceGraph, "source_graph.json", &crate_map)?;
    drop(pb);

    let src_fns = file_data_arr
        .iter()
        .map(|file| extract_generalized_functions_from_file(&file.ast, &file.path))
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect_vec();

    let src_structs = crate_map.root_module().collect_structs();
    let src_enums = crate_map.root_module().collect_enums();
    let src_types = resolve_type_aliases(crate_map.root_module().collect_types());

    let mut type_parser = TypeParser::new(src_structs, src_enums, src_types);
    let mut function_parser = FunctionParser::new(&mut type_parser);

    let ir_funcs = src_fns
        .iter()
        .map(|f| {
            function_parser.parse_function(&f.generalized_item_fn, &f.path, &config.rust_crate_dir)
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        // to give downstream a stable output
        .sorted_by_cached_key(|func| func.name.clone())
        .collect_vec();

    let has_executor = (file_data_arr.iter()).any(|file| parse_has_executor(&file.content));

    let (struct_pool, enum_pool) = type_parser.consume();

    Ok(IrPack {
        funcs: ir_funcs,
        struct_pool,
        enum_pool,
        has_executor,
    })
}

struct FileData {
    path: PathBuf,
    content: String,
    ast: File,
}

fn read_files(
    rust_input_paths: &[PathBuf],
    rust_crate_dir: &Path,
    cached_rust_reader: &mut CachedRustReader,
    dumper: &Dumper,
) -> anyhow::Result<Vec<FileData>> {
    let contents = rust_input_paths
        .iter()
        .map(|rust_input_path| {
            let content =
                cached_rust_reader.read_rust_file(rust_input_path, rust_crate_dir, dumper)?;
            Ok((rust_input_path.to_owned(), content))
        })
        .collect::<anyhow::Result<Vec<(PathBuf, String)>>>()?;

    syn_parse_files(contents)
}

fn syn_parse_files(contents: Vec<(PathBuf, String)>) -> anyhow::Result<Vec<FileData>> {
    (contents.into_iter())
        .map(|(rust_input_path, content)| {
            let ast = syn::parse_file(&content)?;
            Ok(FileData {
                path: rust_input_path,
                content,
                ast,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::codegen::config::internal_config::RustInputPathPack;
    use crate::codegen::dumper::Dumper;
    use crate::codegen::parser::internal_config::ParserInternalConfig;
    use crate::codegen::parser::parse;
    use crate::codegen::parser::reader::CachedRustReader;
    use crate::codegen::parser::source_graph::crates::Crate;
    use crate::utils::logs::configure_opinionated_test_logging;
    use crate::utils::test_utils::{
        create_path_sanitizers, get_test_fixture_dir, json_golden_test,
    };
    use log::info;
    use serial_test::serial;
    use std::path::Path;

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

    fn body(
        fixture_name: &str,
        rust_input_path_pack: Option<Box<dyn Fn(&Path) -> RustInputPathPack>>,
    ) -> anyhow::Result<()> {
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
        )?;

        let actual_ir = parse(
            &ParserInternalConfig {
                rust_input_path_pack: rust_input_path_pack.map(|f| f(&rust_crate_dir)).unwrap_or(
                    RustInputPathPack {
                        rust_input_paths: vec![rust_crate_dir.join("src/api.rs")],
                    },
                ),
                rust_crate_dir: rust_crate_dir.clone(),
            },
            &mut CachedRustReader::default(),
            &Dumper(&Default::default()),
        )?;
        json_golden_test(
            &serde_json::to_value(actual_ir)?,
            &rust_crate_dir.join("expect_ir.json"),
            &[],
        )?;

        Ok(())
    }
}
