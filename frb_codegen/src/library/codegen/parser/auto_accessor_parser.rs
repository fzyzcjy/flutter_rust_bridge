use crate::codegen::config::internal_config::RustInputPathPack;
use crate::codegen::ir::func::IrFunc;
use crate::codegen::ir::ty::IrType;
use crate::codegen::parser::misc::extract_src_types_in_paths;
use crate::codegen::parser::source_graph::modules::Struct;
use crate::codegen::parser::type_parser::TypeParser;
use itertools::Itertools;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub(crate) fn parse_auto_accessors(
    src_structs: &HashMap<String, &Struct>,
    type_parser: &mut TypeParser,
    rust_input_paths: &[PathBuf],
    rust_crate_dir: &Path,
) -> anyhow::Result<Vec<IrFunc>> {
    let src_structs_in_paths =
        extract_src_types_in_paths(src_structs, rust_input_paths, rust_crate_dir)?;
    src_structs_in_paths
        .iter()
        .filter(|struct_name| is_struct_opaque(&struct_name.name, type_parser))
        .flat_map(|struct_name| parse_auto_accessors_of_struct(&struct_name.name, type_parser))
        .collect()
}

fn parse_auto_accessors_of_struct(
    struct_name: &str,
    type_parser: &mut TypeParser,
) -> anyhow::Result<Vec<IrFunc>> {
    TODO
}

fn is_struct_opaque(struct_name: &str, type_parser: &mut TypeParser) -> anyhow::Result<bool> {
    let ty = type_parser.parse_type(syn::parse_str(struct_name)?)?;
    Ok(matches!(ty, IrType::RustAutoOpaqueImplicit(_)))
}
