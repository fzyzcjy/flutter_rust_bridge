use crate::codegen::ir::hir::flat::HirFlatCrate;
use crate::codegen::ir::hir::hierarchical::function::HirFunction;
use crate::codegen::ir::hir::hierarchical::module::HirModule;
use crate::codegen::ir::hir::hierarchical::struct_or_enum::HirEnum;
use crate::codegen::ir::hir::hierarchical::struct_or_enum::HirStruct;
use crate::codegen::parser::hir::flat::type_alias_resolver::resolve_type_aliases;
use log::debug;
use std::collections::HashMap;
use std::fmt::Debug;
use syn::Type;
use crate::codegen::ir::hir::hierarchical::pack::HirPack;

mod type_alias_resolver;

pub(crate) fn parse(hir: &HirPack) -> anyhow::Result<HirFlatCrate> {
    Ok(HirFlatCrate {
        functions: collect_functions(root_module),
        structs: collect_structs(root_module),
        enums: collect_enums(root_module),
        types: resolve_type_aliases(collect_types(root_module)),
        modules: collect_modules(root_module),
    })
}

fn collect_functions(module: &HirModule) -> Vec<&HirFunction> {
    collect_objects_vec(module, |module| module.content.functions.iter().collect())
}

fn collect_modules(module: &HirModule) -> Vec<&HirModule> {
    collect_objects_vec(module, |module| vec![module])
}

fn collect_structs(module: &HirModule) -> HashMap<String, &HirStruct> {
    collect_objects_map(
        module,
        |module| &module.content.structs,
        |x| (x.0.ident.to_string(), x),
    )
}

fn collect_enums(module: &HirModule) -> HashMap<String, &HirEnum> {
    collect_objects_map(
        module,
        |module| &module.content.enums,
        |x| (x.0.ident.to_string(), x),
    )
}

fn collect_types(module: &HirModule) -> HashMap<String, Type> {
    collect_objects_map(
        module,
        |module| &module.content.type_alias,
        |x| (x.ident.clone(), x.target.clone()),
    )
}

fn collect_objects_map<'a, T: 'a, F, G, V: 'a>(
    module: &'a HirModule,
    f: F,
    extract_entry: G,
) -> HashMap<String, V>
where
    F: Fn(&HirModule) -> &[T],
    G: Fn(&'a T) -> (String, V),
    V: Debug,
{
    let mut ans = HashMap::new();
    visit_modules(module, &mut |module| {
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

fn collect_objects_vec<'a, T: 'a, F>(module: &'a HirModule, f: F) -> Vec<T>
where
    F: Fn(&'a HirModule) -> Vec<T>,
{
    let mut ans = vec![];
    visit_modules(module, &mut |module| ans.extend(f(module)));
    ans
}

fn visit_modules<'a, F: FnMut(&'a HirModule)>(module: &'a HirModule, f: &mut F) {
    f(module);
    for scope_module in module.content.modules.iter() {
        visit_modules(scope_module, f);
    }
}
