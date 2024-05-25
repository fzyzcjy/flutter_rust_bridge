pub(crate) mod structs;
mod functions;
mod methods;

use itertools::Itertools;
use syn::File;
use crate::codegen::parser::function_extractor::functions::extract_fns_from_file;
use crate::codegen::parser::function_extractor::methods::extract_methods_from_file;
use crate::codegen::parser::function_extractor::structs::PathAndItemFn;

pub(super) fn extract_generalized_functions_from_file(
    file: &File,
    path: &std::path::Path,
) -> anyhow::Result<Vec<PathAndItemFn>> {
    let item_fns = [
        extract_fns_from_file(file),
        extract_methods_from_file(file)?,
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
