use crate::codegen::hir::hierarchical::struct_or_enum::{Enum, Struct};
use crate::codegen::ir::namespace::{Namespace, NamespacedName};
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::IrType;
use crate::codegen::parser::misc::extract_src_types_in_paths;
use crate::codegen::parser::type_parser::path_data::extract_path_data;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use syn::Type;

pub(crate) fn get_unused_types(
    pack: &IrPack,
    src_structs: &HashMap<String, &Struct>,
    src_enums: &HashMap<String, &Enum>,
    rust_input_namespaces: &[Namespace],
) -> anyhow::Result<Vec<NamespacedName>> {
    let all_types = [
        extract_src_types_in_paths(src_structs, rust_input_namespaces)?,
        extract_src_types_in_paths(src_enums, rust_input_namespaces)?,
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
