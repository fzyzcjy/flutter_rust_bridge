use crate::codegen::ir::hir::tree::module::HirTreeModule;
use crate::codegen::ir::hir::tree::pack::HirTreePack;
use crate::utils::namespace::Namespace;
use itertools::Itertools;
use syn::UseTree;

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
        .flat_map(parse_pub_use_from_item)
        .collect_vec()
}

// the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
// frb-coverage:ignore-start
fn parse_pub_use_from_item(item: &syn::Item) -> Vec<PubUseInfo> {
    // frb-coverage:ignore-end
    if let syn::Item::Use(item_use) = item {
        if matches!(item_use.vis, syn::Visibility::Public(_)) {
            return parse_pub_use_from_use_tree(&item_use.tree);
            // let tree_string = quote::quote!(#tree).to_string().replace(' ', "");
            // let tree_parts = tree_string.split(Namespace::SEP).collect_vec();
            // let name_filters = match *tree_parts.last().unwrap() {
            //     "*" => None,
            //     x => Some(vec![x.to_string()]),
            // };
            //
            // return Some(PubUseInfo {
            //     namespace: Namespace::new(
            //         (tree_parts[..tree_parts.len() - 1].iter())
            //             .map(ToString::to_string)
            //             .collect_vec(),
            //     ),
            //     name_filters,
            // });
        }
    }
    vec![]
}

fn parse_pub_use_from_use_tree(tree: &UseTree) -> Vec<PubUseInfo> {
    match tree {
        UseTree::Path(inner) => (parse_pub_use_from_use_tree(&inner.tree).into_iter())
            .map(|x| PubUseInfo {
                namespace: namespace_add_prefix(&x.namespace, &inner.ident.to_string()),
                name_filter: x.name_filter,
            })
            .collect_vec(),
        UseTree::Name(inner) => vec![PubUseInfo {
            namespace: Namespace::new(vec![]),
            name_filter: Some(inner.ident.to_string()),
        }],
        UseTree::Glob(_) => vec![PubUseInfo {
            namespace: Namespace::new(vec![]),
            name_filter: None,
        }],
        UseTree::Group(inner) => (inner.items.iter())
            .flat_map(parse_pub_use_from_use_tree)
            .collect_vec(),
        // Not supported yet
        // frb-coverage:ignore-start
        UseTree::Rename(_) => vec![],
        // frb-coverage:ignore-end
    }
}

fn namespace_add_prefix(namespace: &Namespace, prefix: &str) -> Namespace {
    let mut chunks = vec![prefix.to_owned()];
    chunks.extend(namespace.path().iter().map(|x| x.to_string()));
    Namespace::new(chunks)
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct PubUseInfo {
    namespace: Namespace,
    name_filter: Option<String>,
}

impl PubUseInfo {
    // the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
    // frb-coverage:ignore-start
    fn is_interest_name(&self, name: &str) -> bool {
        // frb-coverage:ignore-end
        if let Some(name_filters) = &self.name_filter {
            name_filters.contains(&name.to_owned())
        } else {
            true
        }
    }
}

// the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
// frb-coverage:ignore-start
fn transform_module_by_pub_use_single(
    module: &mut HirTreeModule,
    pub_use_info: &PubUseInfo,
) -> anyhow::Result<()> {
    // frb-coverage:ignore-end
    if let Some(src_mod) = module.get_module_nested(&pub_use_info.namespace.path()) {
        // Codecov seems to be buggy by saying this line is not covered (while lines above/below) are
        // frb-coverage:ignore-start
        log::debug!(
            "transform_module_by_pub_use_single pub_use_info={:?}",
            pub_use_info
        );
        // frb-coverage:ignore-end

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
                let is_public_enough = is_item_public(x).unwrap_or(true);

                is_interest_name && is_public_enough && is_localized_definition(x)
            })
            .cloned()
            .collect_vec();

        module.items.extend(src_mod_interest_items);
    } else {
        // Codecov seems to be buggy by saying this line is not covered (while lines above/below) are
        // frb-coverage:ignore-start
        log::debug!(
            "transform_module_by_pub_use_single skip `{pub_use_info:?}` since cannot find mod"
        );
        // frb-coverage:ignore-end
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

pub(crate) fn is_item_public(item: &syn::Item) -> Option<bool> {
    let vis = match item {
        syn::Item::Struct(x) => &x.vis,
        syn::Item::Enum(x) => &x.vis,
        syn::Item::Type(x) => &x.vis,
        syn::Item::Fn(x) => &x.vis,
        syn::Item::Trait(x) => &x.vis,
        _ => return None,
    };
    Some(matches!(vis, syn::Visibility::Public(_)))
}

pub(crate) fn is_localized_definition(item: &syn::Item) -> bool {
    match item {
        syn::Item::Struct(_)
        | syn::Item::Enum(_)
        | syn::Item::Type(_)
        | syn::Item::Fn(_)
        | syn::Item::Trait(_) => true,
        // e.g. `syn::Item::Impl` should *not* be affected
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parse_pub_use_from_item() {
        fn body(code: &str, expect: Vec<PubUseInfo>) {
            let item: syn::Item = syn::parse_str(code).unwrap();
            let actual = parse_pub_use_from_item(&item);
            assert_eq!(actual, expect);
        }

        body(
            "pub use one::two::*;",
            vec![PubUseInfo {
                namespace: Namespace::new_raw("one::two".to_owned()),
                name_filter: None,
            }],
        );

        body(
            "pub use one::two::Three;",
            vec![PubUseInfo {
                namespace: Namespace::new_raw("one::two".to_owned()),
                name_filter: Some("Three".to_owned()),
            }],
        );

        // https://github.com/fzyzcjy/flutter_rust_bridge/issues/2102#issuecomment-2179595124
        body(
            "pub use one::two::{x, y, z};",
            vec![
                PubUseInfo {
                    namespace: Namespace::new_raw("one::two".to_owned()),
                    name_filter: Some("x".to_owned()),
                },
                PubUseInfo {
                    namespace: Namespace::new_raw("one::two".to_owned()),
                    name_filter: Some("y".to_owned()),
                },
                PubUseInfo {
                    namespace: Namespace::new_raw("one::two".to_owned()),
                    name_filter: Some("z".to_owned()),
                },
            ],
        );

        // https://github.com/fzyzcjy/flutter_rust_bridge/issues/2102#issuecomment-2179595124
        body(
            "pub use one::two::{x, u::{v, w}};",
            vec![
                PubUseInfo {
                    namespace: Namespace::new_raw("one::two".to_owned()),
                    name_filter: Some("x".to_owned()),
                },
                PubUseInfo {
                    namespace: Namespace::new_raw("one::two::u".to_owned()),
                    name_filter: Some("v".to_owned()),
                },
                PubUseInfo {
                    namespace: Namespace::new_raw("one::two::u".to_owned()),
                    name_filter: Some("w".to_owned()),
                },
            ],
        );
    }
}
