use crate::codegen::config::internal_config::RustInputPathPack;
use crate::codegen::ir::namespace::Namespace;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::IrType;
use crate::codegen::parser::source_graph::modules::{Enum, Struct, StructOrEnumWrapper};
use clap::builder::Str;
use itertools::Itertools;
use log::warn;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

pub(super) fn sanity_check_unused_struct_enum(
    pack: &IrPack,
    src_structs: &HashMap<String, &Struct>,
    src_enums: &HashMap<String, &Enum>,
    rust_input_path_pack: &RustInputPathPack,
    rust_crate_dir: &PathBuf,
) -> anyhow::Result<()> {
    let interest_input_paths = rust_input_path_pack
        .rust_input_paths
        .iter()
        .map(|p| Namespace::new_from_rust_crate_path(p, rust_crate_dir))
        .collect::<anyhow::Result<Vec<_>>>()?;

    let all_types: HashSet<String> = [
        extract_interest_src_types(src_structs, &interest_input_paths),
        extract_interest_src_types(src_enums, &interest_input_paths),
    ]
    .concat()
    .into_iter()
    .collect();

    let used_types: HashSet<String> = pack
        .distinct_types(None)
        .into_iter()
        .flat_map(|ty| get_potential_struct_or_enum_names(&ty))
        .collect();

    let unused_types = all_types.difference(&used_types).collect_vec();

    if !unused_types.is_empty() {
        warn!(
            "Some structs/enums are exported as `pub`, but are never used in any `pub` functions, thus they are ignored: {:?}",
            unused_types,
        )
    }

    Ok(())
}

fn extract_interest_src_types<T: StructOrEnumWrapper<I>, I>(
    src_items: &HashMap<String, &T>,
    interest_input_paths: &[Namespace],
) -> Vec<String> {
    src_items
        .iter()
        .filter(|(_, v)| interest_input_paths.contains(&v.inner().namespace()))
        .map(|(k, _)| k.to_owned())
        .collect_vec()
}

fn get_potential_struct_or_enum_names(ty: &IrType) -> Vec<String> {
    match ty {
        IrType::StructRef(ty) => vec![ty.ident.0.name.clone()],
        IrType::EnumRef(ty) => vec![ty.ident.0.name.clone()],
        IrType::RustOpaque(ty) => get_potential_struct_or_enum_names_from_str(&ty.inner.0),
        // TODO rm?
        // IrType::Delegate(IrTypeDelegate::PrimitiveEnum(ty)) => vec![ty.ir.ident.0.name.clone()],
        _ => None,
    }
}

fn get_potential_struct_or_enum_names_from_str(ty: &str) -> Vec<Str> {
    todo!()
}
