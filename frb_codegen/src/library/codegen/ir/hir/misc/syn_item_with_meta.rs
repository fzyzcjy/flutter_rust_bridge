use crate::codegen::ir::hir::tree::module::HirTreeModuleMeta;

pub(crate) struct SynItemWithMeta {
    pub meta: HirTreeModuleMeta,
    pub item: syn::Item,
}
