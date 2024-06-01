use crate::codegen::parser::function_extractor::structs::GeneralizedItemFn;
use crate::if_then_some;
use itertools::Itertools;
use syn::*;

pub(super) fn extract_fns_from_file(items: &[syn::Item]) -> Vec<GeneralizedItemFn> {
    (items.iter())
        .filter_map(|item| if_then_some!(let Item::Fn(ref item_fn) = item, item_fn))
        .cloned()
        .map(|item_fn| GeneralizedItemFn::Function { item_fn })
        .collect_vec()
}
