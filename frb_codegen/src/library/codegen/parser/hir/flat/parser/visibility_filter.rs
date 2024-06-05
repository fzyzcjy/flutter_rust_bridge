use crate::codegen::ir::hir::tree::module::HirTreeModuleMeta;
use crate::codegen::parser::hir::flat::parser::flattener::SynItemWithMeta;
use crate::utils::crate_name::CrateName;
use itertools::Itertools;

pub(crate) fn filter_visible_modules(items: Vec<SynItemWithMeta>) -> Vec<SynItemWithMeta> {
    (items.into_iter())
        .filter(|item| is_interest_mod(&item.meta))
        .collect_vec()
}

fn is_interest_mod(meta: &HirTreeModuleMeta) -> bool {
    // If it is third party crate, then we only scan the `pub` mods,
    // since for non-pub modes, it is impossible to use them even if we scanned them.
    meta.namespace.path()[0] == CrateName::SELF_CRATE || meta.is_public()
}
