use crate::codegen::ir::hir::hierarchical::misc::HirCommon;
use crate::codegen::ir::hir::hierarchical::module::HirModule;
use crate::utils::namespace::Namespace;
use itertools::Itertools;
use quote::ToTokens;

pub(crate) fn transform(mut module: HirModule, items: &[syn::Item]) -> anyhow::Result<HirModule> {
    // Only apply to third party crate currently, since in self crate usually no need to care about this
    if module.meta.namespace.crate_name().is_self_crate() {
        return Ok(module);
    }

    let pub_use_infos = parse_pub_use_from_items(items);
    for pub_use_info in pub_use_infos {
        transform_module_by_pub_use_single(&mut module, &pub_use_info)?;
    }
    Ok(module)
}

fn parse_pub_use_from_items(items: &[syn::Item]) -> Vec<PubUseInfo> {
    items
        .iter()
        .filter_map(parse_pub_use_from_item)
        .collect_vec()
}

fn parse_pub_use_from_item(item: &syn::Item) -> Option<PubUseInfo> {
    if let syn::Item::Use(item_use) = item {
        if matches!(item_use.vis, syn::Visibility::Public(_)) {
            let tree = &item_use.tree;
            let tree_string = quote::quote!(#tree).to_string().replace(' ', "");
            let tree_parts = tree_string.split(Namespace::SEP).collect_vec();
            let name_filters = match *tree_parts.last().unwrap() {
                "*" => None,
                x => Some(vec![x.to_string()]),
            };

            return Some(PubUseInfo {
                namespace: Namespace::new(
                    (tree_parts[..tree_parts.len() - 1].iter())
                        .map(ToString::to_string)
                        .collect_vec(),
                ),
                name_filters,
            });
        }
    }
    None
}

#[derive(Debug, Clone)]
struct PubUseInfo {
    namespace: Namespace,
    name_filters: Option<Vec<String>>,
}

impl PubUseInfo {
    fn is_interest_name(&self, name: &str) -> bool {
        if let Some(name_filters) = &self.name_filters {
            name_filters.contains(&name.to_owned())
        } else {
            true
        }
    }
}

fn transform_module_by_pub_use_single(
    module: &mut HirModule,
    pub_use_info: &PubUseInfo,
) -> anyhow::Result<()> {
    if let Some(src_mod) = (module.content).get_module_nested(&pub_use_info.namespace.path()) {
        if src_mod.meta.is_public() {
            log::debug!("transform_module_by_pub_use_single skip `{pub_use_info:?}` since src mod already public");
            return Ok(());
        }

        let self_namespace = &module.meta.namespace;

        let src_functions =
            transform_items(&src_mod.content.functions, self_namespace, pub_use_info);
        let src_structs = transform_items(&src_mod.content.structs, self_namespace, pub_use_info);
        let src_enums = transform_items(&src_mod.content.enums, self_namespace, pub_use_info);
        let src_traits = transform_items(&src_mod.content.traits, self_namespace, pub_use_info);

        module.content.functions.extend(src_functions);
        module.content.structs.extend(src_structs);
        module.content.enums.extend(src_enums);
        module.content.traits.extend(src_traits);
    } else {
        log::debug!(
            "transform_module_by_pub_use_single skip `{pub_use_info:?}` since cannot find mod"
        );
    }

    Ok(())
}

fn transform_items<T: HirCommon>(
    items: &[T],
    self_namespace: &Namespace,
    pub_use_info: &PubUseInfo,
) -> Vec<T> {
    (items.iter())
        .filter(|x| pub_use_info.is_interest_name(&x.ident().to_string()))
        .map(|x| x.with_namespace(self_namespace.clone()))
        .collect_vec()
}
