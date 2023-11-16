pub(crate) mod attribute_parser;
pub(crate) mod function_extractor;
pub(crate) mod function_parser;
pub(crate) mod internal_config;
pub(crate) mod misc;
pub(crate) mod reader;
pub(crate) mod source_graph;
pub(crate) mod type_alias_resolver;
pub(crate) mod type_parser;

use crate::codegen::ir::pack::IrPack;
use crate::codegen::parser::function_extractor::extract_generalized_functions_from_file;
use crate::codegen::parser::function_parser::FunctionParser;
use crate::codegen::parser::internal_config::ParserInternalConfig;
use crate::codegen::parser::misc::parse_has_executor;
use crate::codegen::parser::reader::read_rust_file;
use crate::codegen::parser::type_alias_resolver::resolve_type_aliases;
use crate::codegen::parser::type_parser::TypeParser;
use crate::library::misc::consts::HANDLER_NAME;
use itertools::{sorted, Itertools};
use log::trace;
use std::path::Path;
use syn::File;

// TODO handle multi file correctly
pub(crate) fn parse(config: &ParserInternalConfig) -> anyhow::Result<IrPack> {
    let rust_input_paths = config
        .rust_input_path_pack
        .rust_input_path
        .values()
        .collect_vec();
    trace!("rust_input_paths={:?}", &rust_input_paths);

    let source_rust_contents: Vec<String> = rust_input_paths
        .iter()
        .map(|rust_input_path| read_rust_file(rust_input_path, &config.rust_crate_dir))
        .collect::<anyhow::Result<Vec<_>>>()?;
    let file_asts = source_rust_contents
        .iter()
        .map(|s| syn::parse_file(s))
        .collect::<syn::Result<Vec<_>>>()?;

    let crate_map = source_graph::crates::Crate::parse(&config.rust_crate_dir.join("Cargo.toml"))?;
    // trace!("crate_map={:?}", &crate_map);

    let src_fns = file_asts
        .iter()
        .map(extract_generalized_functions_from_file)
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
        .map(|f| function_parser.parse_function(f))
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        // to give downstream a stable output
        .sorted_by_cached_key(|func| func.name.clone())
        .collect_vec();

    let has_executor = source_rust_contents.iter().any(|s| parse_has_executor(s));

    let (struct_pool, enum_pool) = type_parser.consume();

    Ok(IrPack {
        funcs: ir_funcs,
        struct_pool,
        enum_pool,
        has_executor,
    })
}

#[cfg(test)]
mod tests {
    use crate::codegen::parser::internal_config::{ParserInternalConfig, RustInputPathPack};
    use crate::codegen::parser::parse;
    use crate::codegen::parser::source_graph::crates::Crate;
    use crate::utils::logs::configure_opinionated_test_logging;
    use crate::utils::path_utils::path_to_string;
    use crate::utils::test_utils::{get_test_fixture_dir, json_golden_test};
    use serial_test::serial;
    use std::path::Path;

    #[test]
    #[serial]
    fn test_simple() -> anyhow::Result<()> {
        body("codegen_parser/simple", None)
    }

    #[test]
    #[serial]
    fn test_multi_input_file() -> anyhow::Result<()> {
        body(
            "codegen_parser/multi_input_file",
            Some(Box::new(|rust_crate_dir| RustInputPathPack {
                rust_input_path: [
                    (
                        "namespace_one".to_owned().into(),
                        rust_crate_dir.join("src/api_one.rs"),
                    ),
                    (
                        "namespace_two".to_owned().into(),
                        rust_crate_dir.join("src/api_two.rs"),
                    ),
                ]
                .into(),
            })),
        )
    }

    #[test]
    #[serial]
    fn test_use_type_in_another_file() -> anyhow::Result<()> {
        body("codegen_parser/use_type_in_another_file", None)
    }

    #[test]
    #[serial]
    fn test_qualified_names() -> anyhow::Result<()> {
        body("codegen_parser/qualified_names", None)
    }

    #[test]
    #[serial]
    fn test_non_qualified_names() -> anyhow::Result<()> {
        body("codegen_parser/non_qualified_names", None)
    }

    fn body(
        fixture_name: &str,
        rust_input_path_pack: Option<Box<dyn Fn(&Path) -> RustInputPathPack>>,
    ) -> anyhow::Result<()> {
        configure_opinionated_test_logging();
        let test_fixture_dir = get_test_fixture_dir(fixture_name);
        let rust_crate_dir = test_fixture_dir.clone();

        let crate_map = Crate::parse(&rust_crate_dir.join("Cargo.toml"))?;
        json_golden_test(
            &serde_json::to_value(crate_map)?,
            &rust_crate_dir.join("expect_source_graph.json"),
            &vec![(
                path_to_string(&test_fixture_dir)?,
                "{the-working-directory}".to_owned(),
            )],
        )?;

        let actual_ir = parse(&ParserInternalConfig {
            rust_input_path_pack: rust_input_path_pack.map(|f| f(&rust_crate_dir)).unwrap_or(
                RustInputPathPack {
                    rust_input_path: [(
                        "my_namespace".to_owned().into(),
                        rust_crate_dir.join("src/api.rs"),
                    )]
                    .into(),
                },
            ),
            rust_crate_dir: rust_crate_dir.clone(),
        })?;
        json_golden_test(
            &serde_json::to_value(actual_ir)?,
            &rust_crate_dir.join("expect_ir.json"),
            &[],
        )?;

        Ok(())
    }
}
