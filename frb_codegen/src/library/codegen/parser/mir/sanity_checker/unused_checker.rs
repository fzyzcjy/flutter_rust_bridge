use crate::codegen::ir::hir::flat::struct_or_enum::HirFlatEnum;
use crate::codegen::ir::hir::flat::struct_or_enum::HirFlatStruct;
use crate::codegen::ir::mir::pack::MirPack;
use crate::codegen::ir::mir::ty::delegate::MirTypeDelegate;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::internal_config::RustInputNamespacePack;
use crate::codegen::parser::mir::parser::misc::extract_src_types_in_paths;
use crate::codegen::parser::mir::parser::ty::path_data::extract_path_data;
use crate::utils::namespace::NamespacedName;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use syn::Type;

pub(crate) fn get_unused_types(
    pack: &MirPack,
    src_structs: &HashMap<String, &HirFlatStruct>,
    src_enums: &HashMap<String, &HirFlatEnum>,
    rust_input_namespace_pack: &RustInputNamespacePack,
) -> anyhow::Result<Vec<NamespacedName>> {
    let all_types = [
        extract_src_types_in_paths(src_structs, rust_input_namespace_pack)?,
        extract_src_types_in_paths(src_enums, rust_input_namespace_pack)?,
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

fn get_potential_struct_or_enum_names(ty: &MirType) -> anyhow::Result<Vec<String>> {
    Ok(match ty {
        MirType::StructRef(ty) => vec![ty.ident.0.name.clone()],
        MirType::EnumRef(ty) => vec![ty.ident.0.name.clone()],
        MirType::RustOpaque(ty) => get_potential_struct_or_enum_names_from_syn_type(
            &syn::parse_str(&ty.inner.0.with_static_lifetime())?,
        )?,
        MirType::Delegate(MirTypeDelegate::PrimitiveEnum(ty)) => vec![ty.mir.ident.0.name.clone()],
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
