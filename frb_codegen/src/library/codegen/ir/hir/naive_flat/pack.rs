use crate::codegen::ir::hir::naive_flat::item::HirNaiveFlatItem;

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct HirNaiveFlatPack {
    pub items: Vec<HirNaiveFlatItem>,
}
