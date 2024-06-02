use crate::codegen::ir::hir::hierarchical::module::HirModule;
use itertools::Itertools;

pub(crate) fn transform_module_by_pub_use(
    raw: HirModule,
    items: &[syn::Item],
) -> anyhow::Result<HirModule> {
    TODO
}

fn parse_pub_use_from_items(items: &[syn::Item]) -> Vec<String> {
    items
        .iter()
        .filter_map(parse_pub_use_from_item)
        .collect_vec()
}

fn parse_pub_use_from_item(item: &syn::Item) -> Option<String> {
    if let syn::Item::Use(item_use) = item {
        if matches!(item_use.vis, syn::Visibility::Public(_)) {
            let tree = item_use.tree;
            let tree_string = quote::quote!(#tree).to_string();
            tree_string.strip_suffix("::*")
        }
    }
    None
}
