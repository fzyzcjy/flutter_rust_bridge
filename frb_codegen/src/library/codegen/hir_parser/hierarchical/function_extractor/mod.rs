mod functions;
mod methods;

use crate::codegen::hir::hierarchical::function::HirFunction;
use crate::codegen::ir::namespace::Namespace;
use crate::codegen::parser::function_extractor::functions::extract_fns_from_syn_items;
use crate::codegen::parser::function_extractor::methods::extract_methods_from_syn_items;
use crate::codegen::parser::function_extractor::structs::HirFunction;
use itertools::Itertools;
use syn::File;

pub(super) fn extract_generalized_functions_from_syn_items(
    items: &[syn::Item],
    namespace: &Namespace,
) -> anyhow::Result<Vec<HirFunction>> {
    let item_fns = [
        extract_fns_from_syn_items(items),
        extract_methods_from_syn_items(items)?,
    ]
    .concat();
    let ans = (item_fns.into_iter())
        .map(|inner| HirFunction {
            namespace: namespace.clone(),
            inner,
        })
        .collect_vec();
    Ok(ans)
}
