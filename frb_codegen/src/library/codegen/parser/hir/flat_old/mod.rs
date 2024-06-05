use crate::codegen::ir::hir::flat::HirFlatCrate;
use crate::codegen::ir::hir::hierarchical::function::HirFlatFunction;
use crate::codegen::ir::hir::hierarchical::module::HirModule;
use crate::codegen::ir::hir::hierarchical::pack::HirPack;
use crate::codegen::ir::hir::hierarchical::struct_or_enum::HirFlatEnum;
use crate::codegen::ir::hir::hierarchical::struct_or_enum::HirFlatStruct;
use crate::codegen::ir::hir::hierarchical::traits::HirTrait;
use crate::codegen::parser::hir::flat::type_alias_resolver::resolve_type_aliases;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::utils::crate_name::CrateName;
use log::debug;
use std::collections::HashMap;
use std::fmt::Debug;
use syn::Type;

pub(crate) fn parse<'a>(
    config: &ParserHirInternalConfig,
    hir_pack: &'a HirPack,
) -> anyhow::Result<HirFlatCrate<'a>> {
    Ok(HirFlatCrate {
        functions: collect_functions(config, hir_pack),
        structs: collect_structs(hir_pack),
        enums: collect_enums(hir_pack),
        types: resolve_type_aliases(collect_types(hir_pack)),
        modules: collect_modules(hir_pack),
    })
}

fn collect_modules(hir_pack: &HirPack) -> Vec<&HirModule> {
    collect_objects_vec(hir_pack, |module| vec![module])
}

fn collect_structs(hir_pack: &HirPack) -> HashMap<String, &HirFlatStruct> {
    collect_objects_map(
        hir_pack,
        |module| &module.content.structs,
        |x| (x.name.name.clone(), x),
    )
}

fn collect_enums(hir_pack: &HirPack) -> HashMap<String, &HirFlatEnum> {
    collect_objects_map(
        hir_pack,
        |module| &module.content.enums,
        |x| (x.name.name.clone(), x),
    )
}

fn collect_types(hir_pack: &HirPack) -> HashMap<String, Type> {
    collect_objects_map(
        hir_pack,
        |module| &module.content.type_alias,
        |x| (x.ident.clone(), x.target.clone()),
    )
}
