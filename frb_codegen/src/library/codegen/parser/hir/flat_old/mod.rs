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

fn collect_objects_map<'a, T: 'a, F, G, V: 'a>(
    hir_pack: &'a HirPack,
    f: F,
    extract_entry: G,
) -> HashMap<String, V>
where
    F: Fn(&HirModule) -> &[T],
    G: Fn(&'a T) -> (String, V),
    V: Debug,
{
    let mut ans = HashMap::new();
    hir_pack.visit( &mut |module| {
        if !is_interest_mod(module) {
            return;
        }

        for item in f(module) {
            let (key, value) = extract_entry(item);
            if let Some(old_value) = ans.get(&key) {
                debug!("Same key={key} has multiple values: {old_value:?} (thrown away) and {value:?} (used). This may or may not be a problem.");
            }
            ans.insert(key, value);
        }
    });
    ans
}

fn collect_objects_vec<'a, T: 'a, F>(hir_pack: &'a HirPack, f: F) -> Vec<T>
where
    F: Fn(&'a HirModule) -> Vec<T>,
{
    let mut ans = vec![];
    hir_pack.visit(&mut |module| {
        if !is_interest_mod(module) {
            return;
        }

        ans.extend(f(module))
    });
    ans
}
