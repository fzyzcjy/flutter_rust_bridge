use crate::codegen::ir::hir::tree::module::HirTreeModuleMeta;

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct SynItemWithMeta {
    pub meta: HirTreeModuleMeta,
    #[serde(skip_serializing)]
    pub item: syn::Item,
}
