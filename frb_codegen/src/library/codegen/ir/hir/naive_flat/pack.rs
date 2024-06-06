use crate::codegen::ir::hir::misc::syn_item_with_meta::SynItemWithMeta;

#[derive(Debug, Clone, Default, serde::Serialize)]
pub(crate) struct HirNaiveFlatPack {
    pub items: Vec<SynItemWithMeta>,
}
