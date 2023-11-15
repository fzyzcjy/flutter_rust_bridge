pub(crate) mod error;

use std::path::Path;
use log::debug;
use syn::File;
use crate::codegen::ir::pack::IrPack;

pub(crate) type ParserResult<T = (), E = error::Error> = Result<T, E>;

pub(crate) fn parse(rust_input_path: &Path, manifest_path: &Path) -> ParserResult<IrPack> {
    debug!("Phase: Parse source code to AST");
    let source_rust_content = read_rust_file(rust_input_path);
    let file_ast = syn::parse_file(&source_rust_content)?;

    debug!("Phase: Parse AST to IR");
    parse_ast(&source_rust_content, file_ast, &manifest_path)
}

fn parse_ast(source_rust_content: &str, file_ast: File, manifest_path: &Path) -> ParserResult<IrPack> {
    todo!()
}
