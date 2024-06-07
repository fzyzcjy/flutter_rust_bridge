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
    target.extend(module.items.into_iter().map(|item| HirNaiveFlatItem {
        meta: HirNaiveFlatItemMeta {
            namespace: module.meta.namespace.clone(),
            sources: vec![HirGenerationSource::Normal],
            is_module_public: module.meta.is_public(),
        },
        item,
    }));

    for child_module in module.modules {
        flatten_module(child_module, target);
    }
}
