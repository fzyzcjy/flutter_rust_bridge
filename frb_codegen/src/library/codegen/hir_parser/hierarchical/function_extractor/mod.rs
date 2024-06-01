mod functions;
mod methods;
pub(crate) mod structs;

use crate::codegen::parser::function_extractor::functions::extract_fns_from_syn_items;
use crate::codegen::parser::function_extractor::methods::extract_methods_from_syn_items;
use crate::codegen::parser::function_extractor::structs::PathAndItemFn;
use itertools::Itertools;
use syn::File;

pub(super) fn extract_generalized_functions_from_syn_items(
    items: &[syn::Item],
    path: &std::path::Path,
) -> anyhow::Result<Vec<PathAndItemFn>> {
    let item_fns = [
        extract_fns_from_syn_items(items),
        extract_methods_from_syn_items(items)?,
    ]
    .concat();
    let ans = item_fns
        .into_iter()
        .map(|generalized_item_fn| PathAndItemFn {
            path: path.to_owned(),
            generalized_item_fn,
        })
        .collect_vec();
    Ok(ans)
}
