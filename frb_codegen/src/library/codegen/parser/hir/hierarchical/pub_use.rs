use crate::codegen::ir::hir::hierarchical::module::HirModule;
use crate::utils::namespace::Namespace;
use itertools::Itertools;

pub(crate) fn transform_module_by_pub_use(
    mut module: HirModule,
    items: &[syn::Item],
) -> anyhow::Result<HirModule> {
    let pub_use_names = parse_pub_use_from_items(items);
    for pub_use_name in pub_use_names {
        transform_module_by_pub_use_single(&mut module, &pub_use_name)?;
    }
    Ok(module)
}

fn parse_pub_use_from_items(items: &[syn::Item]) -> Vec<Namespace> {
    items
        .iter()
        .filter_map(parse_pub_use_from_item)
        .collect_vec()
}

fn parse_pub_use_from_item(item: &syn::Item) -> Option<Namespace> {
    if let syn::Item::Use(item_use) = item {
        if matches!(item_use.vis, syn::Visibility::Public(_)) {
            let tree = &item_use.tree;
            let tree_string = quote::quote!(#tree).to_string();
            if let Some(interest_use_part) = tree_string.strip_suffix("::*") {
                return Some(Namespace::new_raw(interest_use_part.to_owned()));
            }
        }
    }
    None
}

fn transform_module_by_pub_use_single(
    module: &mut HirModule,
    pub_use_name: &Namespace,
) -> anyhow::Result<()> {
    let target_mod = module.content.get_module_nested(&pub_use_name.path());

    module.content.functions.extend(target_mod.content.functions.clone());
    module.content.structs.extend(target_mod.content.structs.clone());
    module.content.enums.extend(target_mod.content.enums.clone());

    Ok(())
}
