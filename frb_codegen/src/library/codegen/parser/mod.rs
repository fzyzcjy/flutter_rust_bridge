pub(crate) mod attribute_parser;
pub(crate) mod error;
pub(crate) mod function_extractor;
pub(crate) mod function_parser;
pub(crate) mod internal_config;
pub(crate) mod reader;
pub(crate) mod source_graph;
pub(crate) mod type_alias_resolver;
pub(crate) mod type_parser;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::parser::function_extractor::extract_generalized_functions_from_file;
use crate::codegen::parser::function_parser::FunctionParser;
use crate::codegen::parser::internal_config::ParserInternalConfig;
use crate::codegen::parser::reader::read_rust_file;
use crate::codegen::parser::type_alias_resolver::resolve_type_aliases;
use crate::codegen::parser::type_parser::TypeParser;
use crate::library::misc::consts::HANDLER_NAME;
use std::path::Path;
use syn::File;

pub(crate) type ParserResult<T = (), E = error::Error> = Result<T, E>;

// TODO handle multi file correctly
pub(crate) fn parse(config: &ParserInternalConfig) -> ParserResult<IrPack> {
    let raw_packs = config
        .rust_input_path_pack
        .rust_input_path
        .iter()
        // TODO handle namespace
        .map(|(_namespace, rust_input_path)| parse_one(rust_input_path, &config.rust_crate_dir))
        .collect::<Result<Vec<_>, _>>()?;
    let merged_pack = raw_packs.into_iter().reduce(|acc, e| acc.merge(e)).unwrap();
    Ok(merged_pack)
}

fn parse_one(rust_input_path: &Path, rust_crate_dir: &Path) -> ParserResult<IrPack> {
    let source_rust_content = read_rust_file(rust_input_path, rust_crate_dir)?;
    let file_ast = syn::parse_file(&source_rust_content)?;
    parse_one_ast(&source_rust_content, file_ast, &rust_crate_dir)
}

fn parse_one_ast(
    source_rust_content: &str,
    file_ast: File,
    rust_crate_dir: &Path,
) -> ParserResult<IrPack> {
    let crate_map = source_graph::crates::Crate::parse(&rust_crate_dir.join("Cargo.toml"))?;

    let src_fns = extract_generalized_functions_from_file(&file_ast)?;
    let src_structs = crate_map.root_module().collect_structs();
    let src_enums = crate_map.root_module().collect_enums();
    let src_types = resolve_type_aliases(crate_map.root_module().collect_types());

    let mut type_parser = TypeParser::new(src_structs, src_enums, src_types);
    let mut function_parser = FunctionParser::new(&mut type_parser);

    let ir_funcs = src_fns
        .iter()
        .map(|f| function_parser.parse_function(f))
        .collect::<ParserResult<_>>()?;

    let has_executor = source_rust_content.contains(HANDLER_NAME);

    drop(function_parser);

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
    use crate::utils::logs::configure_opinionated_test_logging;
    use crate::utils::test_utils::{get_test_fixture_dir, json_golden_test};
    use serial_test::serial;

    // TODO more tests
    // TODO `chrono::Duration` and `Duration` test
    // TODO `Result`, `anyhow::Result`, `std::result::Result`
    #[test]
    #[serial]
    fn test_simple() -> anyhow::Result<()> {
        body("codegen_parser/simple")
    }

    fn body(fixture_name: &str) -> anyhow::Result<()> {
        configure_opinionated_test_logging();
        let test_fixture_dir = get_test_fixture_dir(fixture_name);

        let actual = parse(&ParserInternalConfig {
            rust_input_path_pack: RustInputPathPack {
                rust_input_path: [(
                    "my_namespace".to_owned().into(),
                    test_fixture_dir.join("src/api.rs"),
                )]
                .into(),
            },
            rust_crate_dir: test_fixture_dir,
        })?;

        json_golden_test(&serde_json::to_value(actual)?, "expect_output.json".into())?;

        Ok(())
    }
}
