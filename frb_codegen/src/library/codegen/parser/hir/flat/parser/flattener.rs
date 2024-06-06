use crate::codegen::ir::hir::tree::module::{HirTreeModule, HirTreeModuleMeta};
use crate::codegen::ir::hir::tree::pack::HirTreePack;

pub(crate) fn flatten(pack: HirTreePack) -> anyhow::Result<Vec<SynItemWithMeta>> {
    let mut ans = vec![];
    for hir_crate in pack.crates {
        flatten_module(hir_crate.root_module, &mut ans);
    }
    Ok(ans)
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
