use crate::codegen::parser::hir::flat::parser::flattener::SynItemWithMeta;
use itertools::Itertools;

pub(crate) fn filter_visible_modules(items: Vec<SynItemWithMeta>) -> Vec<SynItemWithMeta> {
    (items.into_iter()).filter(|item| TODO).collect_vec()
}
