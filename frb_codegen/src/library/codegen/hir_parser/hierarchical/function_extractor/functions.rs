use crate::codegen::parser::function_extractor::structs::HirFunction;
use crate::if_then_some;
use itertools::Itertools;
use syn::*;

pub(super) fn extract_fns_from_syn_items(items: &[syn::Item]) -> Vec<HirFunction> {
    (items.iter())
        .filter_map(|item| if_then_some!(let Item::Fn(ref item_fn) = item, item_fn))
        .cloned()
        .map(|item_fn| HirFunction::Function { item_fn })
        .collect_vec()
}
