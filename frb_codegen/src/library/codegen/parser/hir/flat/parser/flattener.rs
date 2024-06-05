use crate::codegen::ir::hir::tree::module::{HirTreeModule, HirTreeModuleMeta};
use crate::codegen::ir::hir::tree::pack::HirTreePack;

pub(crate) fn flatten(pack: HirTreePack) -> anyhow::Result<Vec<SynItemWithMeta>> {
    TODO;
}

pub(crate) struct SynItemWithMeta {
    pub meta: HirTreeModuleMeta,
    pub item: syn::Item,
}
