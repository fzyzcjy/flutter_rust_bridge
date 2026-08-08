use crate::codegen::ir::hir::misc::generation_source::HirGenerationSource;
use crate::codegen::ir::hir::naive_flat::item::{HirNaiveFlatItem, HirNaiveFlatItemMeta};
use crate::codegen::ir::hir::naive_flat::pack::HirNaiveFlatPack;
use crate::codegen::ir::hir::tree::module::HirTreeModule;
use crate::codegen::ir::hir::tree::pack::HirTreePack;

pub(crate) fn parse(pack: HirTreePack) -> anyhow::Result<HirNaiveFlatPack> {
    let mut items = vec![];
    for hir_crate in pack.crates {
        flatten_module(hir_crate.root_module, &mut items);
    }
    Ok(HirNaiveFlatPack { items })
}

fn flatten_module(module: HirTreeModule, target: &mut Vec<HirNaiveFlatItem>) {
    let imports = module
        .items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Use(item_use) => Some(item_use.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    let mut item_contexts = module.item_contexts;
    item_contexts.resize(module.items.len(), None);
    target.extend(
        module
            .items
            .into_iter()
            .zip(item_contexts)
            .map(|(item, context)| HirNaiveFlatItem {
                meta: HirNaiveFlatItemMeta {
                    namespace: module.meta.namespace.clone(),
                    declaration_namespace: context.as_ref().map_or_else(
                        || module.meta.namespace.clone(),
                        |x| x.declaration_namespace.clone(),
                    ),
                    sources: vec![HirGenerationSource::Normal],
                    is_module_public: module.meta.is_public(),
                    is_module_accessible_from_rust_output: module
                        .meta
                        .is_accessible_from_rust_output,
                    imports: context.map_or_else(|| imports.clone(), |x| x.imports),
                },
                item,
            }),
    );

    for child_module in module.modules {
        flatten_module(child_module, target);
    }
}
