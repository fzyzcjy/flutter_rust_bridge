use crate::codegen::ir::hir::hierarchical::module::HirModule;
use crate::utils::namespace::Namespace;
use itertools::Itertools;

pub(crate) fn transform(mut module: HirModule, items: &[syn::Item]) -> anyhow::Result<HirModule> {
    // Only apply to third party crate currently, since in self crate usually no need to care about this
    if module.meta.namespace.crate_name().is_self_crate() {
        return Ok(module);
    }

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
            let tree_string = quote::quote!(#tree).to_string().replace(' ', "");
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
    if let Some(src_mod) = module.content.get_module_nested(&pub_use_name.path()) {
        let self_namespace = &module.meta.namespace;

        let src_functions = (src_mod.content.functions.iter())
            .map(|x| x.with_namespace(self_namespace.clone()))
            .collect_vec();
        let src_structs = (src_mod.content.structs.iter())
            .map(|x| x.with_namespace(self_namespace.clone()))
            .collect_vec();
        let src_enums = (src_mod.content.enums.iter())
            .map(|x| x.with_namespace(self_namespace.clone()))
            .collect_vec();
        let src_traits = (src_mod.content.traits.iter())
            .map(|x| x.with_namespace(self_namespace.clone()))
            .collect_vec();

        module.content.functions.extend(src_functions);
        module.content.structs.extend(src_structs);
        module.content.enums.extend(src_enums);
        module.content.traits.extend(src_traits);
    } else {
        log::debug!(
            "transform_module_by_pub_use_single skip `{pub_use_name}` since cannot find mod"
        );
    }

    Ok(())
}
