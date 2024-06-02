use crate::codegen::ir::hir::flat::HirFlatCrate;
use crate::codegen::ir::hir::hierarchical::function::HirFunction;
use crate::codegen::ir::hir::hierarchical::module::HirModule;
use crate::codegen::ir::hir::hierarchical::pack::HirPack;
use crate::codegen::ir::hir::hierarchical::struct_or_enum::HirEnum;
use crate::codegen::ir::hir::hierarchical::struct_or_enum::HirStruct;
use crate::codegen::parser::hir::flat::type_alias_resolver::resolve_type_aliases;
use log::debug;
use std::collections::HashMap;
use std::fmt::Debug;
use syn::Type;

mod type_alias_resolver;

pub(crate) fn parse(hir_pack: &HirPack) -> anyhow::Result<HirFlatCrate> {
    Ok(HirFlatCrate {
        functions: collect_functions(hir_pack),
        structs: collect_structs(hir_pack),
        enums: collect_enums(hir_pack),
        types: resolve_type_aliases(collect_types(hir_pack)),
        modules: collect_modules(hir_pack),
    })
}

fn collect_functions(hir_pack: &HirPack) -> Vec<&HirFunction> {
    collect_objects_vec(hir_pack, |module| module.content.functions.iter().collect())
}

fn collect_modules(hir_pack: &HirPack) -> Vec<&HirModule> {
    collect_objects_vec(hir_pack, |module| vec![module])
}

fn collect_structs(hir_pack: &HirPack) -> HashMap<String, &HirStruct> {
    collect_objects_map(
        hir_pack,
        |module| &module.content.structs,
        |x| (x.0.ident.to_string(), x),
    )
}

fn collect_enums(hir_pack: &HirPack) -> HashMap<String, &HirEnum> {
    collect_objects_map(
        hir_pack,
        |module| &module.content.enums,
        |x| (x.0.ident.to_string(), x),
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
    visit_pack(hir_pack, &mut |module| {
        for item in f(module) {
            let (key, value) = extract_entry(item);
            if let Some(old_value) = ans.get(&key) {
                debug!("Same key={key} has multiple values: {old_value:?} (thrown away) and {value:?} (used). This may or may not be a problem.");
            }
            let _old_value = ans.insert(key, value);
        }
    });
    ans
}

fn collect_objects_vec<'a, T: 'a, F>(hir_pack: &'a HirPack, f: F) -> Vec<T>
where
    F: Fn(&'a HirModule) -> Vec<T>,
{
    let mut ans = vec![];
    visit_pack(hir_pack, &mut |module| ans.extend(f(module)));
    ans
}

fn visit_pack<'a, F: FnMut(&'a HirModule)>(hir_pack: &'a HirPack, f: &mut F) {
    for hir_crate in hir_pack.crates.values() {
        visit_modules(&hir_crate.root_module, f);
    }
}

fn visit_modules<'a, F: FnMut(&'a HirModule)>(module: &'a HirModule, f: &mut F) {
    f(module);
    for scope_module in module.content.modules.iter() {
        visit_modules(scope_module, f);
    }
}
