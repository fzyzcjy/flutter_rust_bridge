use crate::codegen::config::internal_config::RustInputPathPack;
use crate::codegen::ir::namespace::{Namespace, NamespacedName};
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::IrType;
use crate::codegen::parser::source_graph::modules::{Enum, Struct, StructOrEnumWrapper};
use crate::codegen::parser::type_parser::path_data::extract_path_data;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use syn::Type;

pub(super) fn get_unused_types(
    pack: &IrPack,
    src_structs: &HashMap<String, &Struct>,
    src_enums: &HashMap<String, &Enum>,
    rust_input_path_pack: &RustInputPathPack,
    rust_crate_dir: &Path,
) -> anyhow::Result<Vec<NamespacedName>> {
    let interest_input_paths = rust_input_path_pack
        .rust_input_paths
        .iter()
        .map(|p| Namespace::new_from_rust_crate_path(p, rust_crate_dir))
        .collect::<anyhow::Result<Vec<_>>>()?;

    let all_types = [
        extract_interest_src_types(src_structs, &interest_input_paths),
        extract_interest_src_types(src_enums, &interest_input_paths),
    ]
    .concat();

    let used_types = pack
        .distinct_types(None)
        .into_iter()
        .map(|ty| get_potential_struct_or_enum_names(&ty))
        .flatten_ok()
        .collect::<anyhow::Result<HashSet<String>>>()?;

    let unused_types = (all_types.into_iter())
        .filter(|ty| !used_types.contains(&ty.name))
        .collect_vec();

    Ok(unused_types)
}

fn extract_interest_src_types<T: StructOrEnumWrapper<I>, I>(
    src_items: &HashMap<String, &T>,
    interest_input_paths: &[Namespace],
) -> Vec<NamespacedName> {
    (src_items.iter())
        .filter_map(|(k, v)| {
            let namespace = v.inner().namespace();
            if interest_input_paths.contains(&namespace) {
                Some(NamespacedName::new(namespace, k.to_owned()))
            } else {
                None
            }
        })
        .collect_vec()
}

fn get_potential_struct_or_enum_names(ty: &IrType) -> anyhow::Result<Vec<String>> {
    Ok(match ty {
        IrType::StructRef(ty) => vec![ty.ident.0.name.clone()],
        IrType::EnumRef(ty) => vec![ty.ident.0.name.clone()],
        IrType::RustOpaque(ty) => {
            get_potential_struct_or_enum_names_from_syn_type(&syn::parse_str(&ty.inner.0)?)?
        }
        IrType::Delegate(IrTypeDelegate::PrimitiveEnum(ty)) => vec![ty.ir.ident.0.name.clone()],
        _ => vec![],
    })
}

fn get_potential_struct_or_enum_names_from_syn_type(ty: &Type) -> anyhow::Result<Vec<String>> {
    Ok(match ty {
        Type::Path(path) => {
            let segments = extract_path_data(&path.path)?;
            let segment = segments.last().unwrap();
            [
                vec![segment.ident.to_owned()],
                (segment.args.iter())
                    .map(get_potential_struct_or_enum_names_from_syn_type)
                    .flatten_ok()
                    .collect::<anyhow::Result<Vec<_>>>()?,
            ]
            .concat()
        }
        Type::Reference(reference) => {
            get_potential_struct_or_enum_names_from_syn_type(&reference.elem)?
        }
        // ... maybe more ...
        _ => vec![],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_potential_struct_or_enum_names_from_syn_type() {
        fn body(s: &str, matcher: Vec<&str>) {
            assert_eq!(
                get_potential_struct_or_enum_names_from_syn_type(&syn::parse_str(s).unwrap())
                    .unwrap(),
                matcher.into_iter().map(|x| x.to_owned()).collect_vec(),
            );
        }

        body("Something", vec!["Something"]);
        body("One<Two>", vec!["One", "Two"]);
        body("a::b::One<c::d::Two>", vec!["One", "Two"]);
        body("&One", vec!["One"]);
        body("&mut One", vec!["One"]);
    }
}
