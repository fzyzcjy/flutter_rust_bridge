use crate::codegen::ir::hir::misc::syn_item_with_meta::SynItemWithMeta;
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

fn flatten_module(module: HirTreeModule, target: &mut Vec<SynItemWithMeta>) {
    target.extend(module.items.into_iter().map(|item| SynItemWithMeta {
        meta: module.meta.clone(),
        item,
    }));

    for child_module in module.modules {
        flatten_module(child_module, target);
    }
}
