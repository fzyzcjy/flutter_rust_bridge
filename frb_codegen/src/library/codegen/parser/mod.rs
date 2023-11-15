pub(crate) mod attribute_parser;
pub(crate) mod error;
pub(crate) mod function_extractor;
pub(crate) mod function_parser;
pub(crate) mod internal_config;
pub(crate) mod reader;
pub(crate) mod source_graph;
pub(crate) mod type_alias_resolver;
pub(crate) mod type_parser;
pub(crate) mod unencodable;

use crate::codegen::ir::pack::IrPack;
use crate::codegen::parser::function_extractor::extract_generalized_functions_from_file;
use crate::codegen::parser::function_parser::FunctionParser;
use crate::codegen::parser::internal_config::ParserInternalConfig;
use crate::codegen::parser::reader::read_rust_file;
use crate::codegen::parser::type_alias_resolver::resolve_type_aliases;
use crate::codegen::parser::type_parser::TypeParser;
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
        .map(|(namespace, rust_input_path)| parse_one(rust_input_path, &config.rust_crate_dir))
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
    let src_types = crate_map.root_module().collect_types();
    let src_types = resolve_type_aliases(src_types);

    let type_parser = TypeParser::new(src_structs, src_enums, src_types);
    let function_parser = FunctionParser::new(&type_parser);

    let funcs = src_fns
        .iter()
        .map(function_parser.parse_function)
        .collect::<ParserResult<_>>()?;

    let has_executor = source_rust_content.contains(HANDLER_NAME);

    let (struct_pool, enum_pool) = type_parser.consume();

    Ok(IrPack {
        funcs,
        struct_pool,
        enum_pool,
        has_executor,
    })
}
