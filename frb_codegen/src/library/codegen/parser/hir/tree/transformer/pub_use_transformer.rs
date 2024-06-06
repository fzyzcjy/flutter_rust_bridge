use crate::codegen::ir::hir::tree::module::HirTreeModule;
use crate::codegen::ir::hir::tree::pack::HirTreePack;
use crate::utils::namespace::Namespace;
use itertools::Itertools;

pub(crate) fn transform(mut pack: HirTreePack) -> anyhow::Result<HirTreePack> {
    for hir_crate in pack.crates.iter_mut() {
        transform_module(&mut hir_crate.root_module)?;
    }
    Ok(pack)
}

fn transform_module(module: &mut HirTreeModule) -> anyhow::Result<()> {
    // Transform child modules *first*, since parent module may `pub use` something in child module
    for child_module in module.modules.iter_mut() {
        transform_module(child_module)?;
    }

    // Only apply to third party crate currently, since in self crate usually no need to care about this
    if module.meta.namespace.crate_name().is_self_crate() {
        return Ok(());
    }

    let pub_use_infos = parse_pub_use_from_items(&module.items);
    for pub_use_info in pub_use_infos {
        transform_module_by_pub_use_single(module, &pub_use_info)?;
    }
    Ok(())
}

fn parse_pub_use_from_items(items: &[syn::Item]) -> Vec<PubUseInfo> {
    (items.iter())
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
    module: &mut HirTreeModule,
    pub_use_info: &PubUseInfo,
) -> anyhow::Result<()> {
    if let Some(src_mod) = module.get_module_nested(&pub_use_info.namespace.path()) {
        log::debug!(
            "transform_module_by_pub_use_single pub_use_info={:?}",
            pub_use_info
        );

        if src_mod.meta.is_public() {
            log::debug!("transform_module_by_pub_use_single skip `{pub_use_info:?}` since src mod already public");
            return Ok(());
        }

        // let self_namespace = &module.meta.namespace;

        let src_mod_interest_items = (src_mod.items.iter())
            .filter(|x| {
                let name_for_use_stmt =
                    name_for_use_stmt(x).unwrap_or_else(|| "NOT_EXIST_NAME".to_owned());
                let is_interest_name = pub_use_info.is_interest_name(&name_for_use_stmt);
                let is_public_enough = is_public_enough(x).unwrap_or(true);

                is_interest_name && is_public_enough
            })
            .cloned()
            .collect_vec();

        module.items.extend(src_mod_interest_items);
    } else {
        log::debug!(
            "transform_module_by_pub_use_single skip `{pub_use_info:?}` since cannot find mod"
        );
    }

    Ok(())
}

fn name_for_use_stmt(item: &syn::Item) -> Option<String> {
    let ident = match item {
        syn::Item::Struct(x) => &x.ident,
        syn::Item::Enum(x) => &x.ident,
        syn::Item::Type(x) => &x.ident,
        syn::Item::Fn(x) => &x.sig.ident,
        syn::Item::Trait(x) => &x.ident,
        _ => return None,
    };
    Some(ident.to_string())
}

fn is_public_enough(item: &syn::Item) -> Option<bool> {
    let vis = match item {
        syn::Item::Struct(x) => &x.vis,
        syn::Item::Enum(x) => &x.vis,
        syn::Item::Type(x) => &x.vis,
        syn::Item::Fn(x) => &x.vis,
        syn::Item::Trait(x) => &x.vis,
        syn::Item::Impl(x) => TODO,
        _ => return None,
    };
    Some(matches!(vis, syn::Visibility::Public(_)))
}
