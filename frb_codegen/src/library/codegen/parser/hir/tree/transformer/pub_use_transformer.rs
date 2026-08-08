use crate::codegen::ir::hir::misc::visibility::is_visibility_accessible_from;
use crate::codegen::ir::hir::tree::module::{HirTreeItemContext, HirTreeModule};
use crate::codegen::ir::hir::tree::pack::HirTreePack;
use crate::utils::namespace::Namespace;
use itertools::Itertools;
use std::collections::HashSet;
use syn::UseTree;

pub(crate) fn transform(
    mut pack: HirTreePack,
    rust_output_path_namespace: &Namespace,
) -> anyhow::Result<HirTreePack> {
    for hir_crate in pack.crates.iter_mut() {
        if hir_crate.name.is_self_crate() {
            transform_self_crate(&mut hir_crate.root_module, rust_output_path_namespace)?;
        } else {
            transform_module(&mut hir_crate.root_module, rust_output_path_namespace)?;
        }
    }
    Ok(pack)
}

#[derive(Clone)]
struct SelfCratePubUse {
    destination_namespace: Namespace,
    info: PubUseInfo,
}

fn transform_self_crate(
    root_module: &mut HirTreeModule,
    rust_output_path_namespace: &Namespace,
) -> anyhow::Result<()> {
    let mut directives = vec![];
    collect_self_crate_pub_uses(root_module, rust_output_path_namespace, &mut directives);
    for _ in 0..=directives.len() {
        let mut changed = false;
        for directive in &directives {
            changed |=
                transform_self_crate_pub_use(root_module, directive, rust_output_path_namespace)?;
        }
        if !changed {
            break;
        }
    }
    Ok(())
}

fn collect_self_crate_pub_uses(
    module: &HirTreeModule,
    rust_output_path_namespace: &Namespace,
    output: &mut Vec<SelfCratePubUse>,
) {
    for child_module in &module.modules {
        collect_self_crate_pub_uses(child_module, rust_output_path_namespace, output);
    }
    for mut info in parse_pub_use_from_items(
        &module.items,
        &module.meta.namespace,
        rust_output_path_namespace,
    ) {
        if let Some(namespace) = resolve_pub_use_namespace(&info.namespace, &module.meta.namespace)
        {
            info.namespace = namespace;
            output.push(SelfCratePubUse {
                destination_namespace: module.meta.namespace.clone(),
                info,
            });
        }
    }
}

fn transform_self_crate_pub_use(
    root_module: &mut HirTreeModule,
    directive: &SelfCratePubUse,
    rust_output_path_namespace: &Namespace,
) -> anyhow::Result<bool> {
    let root_namespace = root_module.meta.namespace.clone();
    if directive.info.namespace == directive.destination_namespace
        || !root_namespace.is_prefix_of(&directive.info.namespace)
        || !root_namespace.is_prefix_of(&directive.destination_namespace)
    {
        return Ok(false);
    }
    let source_path = directive.info.namespace.strip_prefix(&root_namespace);
    let destination_path = directive
        .destination_namespace
        .strip_prefix(&root_namespace);
    let moved_items = {
        let Some(source_module) = root_module.get_module_nested_mut(&source_path.path()) else {
            return Ok(false);
        };
        if source_module.meta.is_accessible_from_rust_output {
            return Ok(false);
        }
        let source_namespace = source_module.meta.namespace.clone();
        let source_imports = source_module
            .items
            .iter()
            .filter_map(|item| match item {
                syn::Item::Use(item_use) => Some(item_use.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();
        source_module
            .item_contexts
            .resize(source_module.items.len(), None);
        let source_items = std::mem::take(&mut source_module.items);
        let source_contexts = std::mem::take(&mut source_module.item_contexts);
        let interest_names = source_items
            .iter()
            .filter(|item| {
                is_interest_item(
                    item,
                    &directive.info,
                    Some((&source_namespace, rust_output_path_namespace)),
                )
            })
            .filter_map(name_for_use_stmt)
            .collect::<HashSet<_>>();
        let mut moved_items = vec![];
        for (mut item, context) in source_items.into_iter().zip(source_contexts) {
            if is_item_or_impl_interesting(&item, &interest_names, directive.info.rename.is_some())
            {
                rename_item_for_use(&mut item, &directive.info);
                moved_items.push((
                    item,
                    context.unwrap_or_else(|| HirTreeItemContext {
                        declaration_namespace: source_namespace.clone(),
                        imports: source_imports.clone(),
                    }),
                ));
            } else {
                source_module.items.push(item);
                source_module.item_contexts.push(context);
            }
        }
        moved_items
    };
    if moved_items.is_empty() {
        return Ok(false);
    }
    let Some(destination_module) = root_module.get_module_nested_mut(&destination_path.path())
    else {
        return Ok(false);
    };
    for (item, context) in moved_items {
        destination_module.items.push(item);
        destination_module.item_contexts.push(Some(context));
    }
    Ok(true)
}

fn resolve_pub_use_namespace(
    namespace: &Namespace,
    declaration_namespace: &Namespace,
) -> Option<Namespace> {
    let segments = namespace
        .path()
        .into_iter()
        .map(ToString::to_string)
        .collect_vec();
    let mut output = declaration_namespace
        .path()
        .into_iter()
        .map(ToString::to_string)
        .collect_vec();
    let mut index = 0;
    match segments.first().map(String::as_str) {
        Some("crate") => {
            output = vec!["crate".to_owned()];
            index = 1;
        }
        Some("self") => index = 1,
        Some("super") => {
            while segments.get(index).map(String::as_str) == Some("super") {
                output.pop()?;
                index += 1;
            }
        }
        _ => {}
    }
    output.extend(segments[index..].iter().cloned());
    Some(Namespace::new(output))
}

fn transform_module(
    module: &mut HirTreeModule,
    rust_output_path_namespace: &Namespace,
) -> anyhow::Result<()> {
    // Transform child modules *first*, since parent module may `pub use` something in child module
    for child_module in module.modules.iter_mut() {
        transform_module(child_module, rust_output_path_namespace)?;
    }

    let pub_use_infos = parse_pub_use_from_items(
        &module.items,
        &module.meta.namespace,
        rust_output_path_namespace,
    );
    for pub_use_info in pub_use_infos {
        transform_module_by_pub_use_single(module, &pub_use_info, rust_output_path_namespace)?;
    }
    Ok(())
}

fn parse_pub_use_from_items(
    items: &[syn::Item],
    declaration_namespace: &Namespace,
    rust_output_path_namespace: &Namespace,
) -> Vec<PubUseInfo> {
    (items.iter())
        .flat_map(|item| {
            parse_pub_use_from_item(item, declaration_namespace, rust_output_path_namespace)
        })
        .collect_vec()
}

// the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
// frb-coverage:ignore-start
fn parse_pub_use_from_item(
    item: &syn::Item,
    declaration_namespace: &Namespace,
    rust_output_path_namespace: &Namespace,
) -> Vec<PubUseInfo> {
    // frb-coverage:ignore-end
    if let syn::Item::Use(item_use) = item {
        if is_visibility_accessible_from(
            &item_use.vis,
            declaration_namespace,
            rust_output_path_namespace,
        ) {
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
                rename: x.rename,
            })
            .collect_vec(),
        UseTree::Name(inner) => vec![PubUseInfo {
            namespace: Namespace::new(vec![]),
            name_filter: Some(inner.ident.to_string()),
            rename: None,
        }],
        UseTree::Glob(_) => vec![PubUseInfo {
            namespace: Namespace::new(vec![]),
            name_filter: None,
            rename: None,
        }],
        UseTree::Group(inner) => (inner.items.iter())
            .flat_map(parse_pub_use_from_use_tree)
            .collect_vec(),
        UseTree::Rename(inner) => vec![PubUseInfo {
            namespace: Namespace::new(vec![]),
            name_filter: Some(inner.ident.to_string()),
            rename: Some(inner.rename.to_string()),
        }],
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
    rename: Option<String>,
}

impl PubUseInfo {
    // the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
    // frb-coverage:ignore-start
    fn is_interest_name(&self, name: &str) -> bool {
        // frb-coverage:ignore-end
        if let Some(name_filters) = &self.name_filter {
            name_filters == name
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
    rust_output_path_namespace: &Namespace,
) -> anyhow::Result<()> {
    // frb-coverage:ignore-end
    let is_self_crate = module.meta.namespace.crate_name().is_self_crate();
    let mut src_mod_interest_items = if let Some(src_mod) =
        module.get_module_nested_mut(&pub_use_info.namespace.path())
    {
        // Codecov seems to be buggy by saying this line is not covered (while lines above/below) are
        // frb-coverage:ignore-start
        log::debug!(
            "transform_module_by_pub_use_single pub_use_info={:?}",
            pub_use_info
        );
        // frb-coverage:ignore-end

        if if is_self_crate {
            src_mod.meta.is_accessible_from_rust_output
        } else {
            src_mod.meta.is_public()
        } {
            log::debug!("transform_module_by_pub_use_single skip `{pub_use_info:?}` since src mod already public");
            return Ok(());
        }

        // let self_namespace = &module.meta.namespace;

        if is_self_crate {
            let src_namespace = src_mod.meta.namespace.clone();
            let interest_names = src_mod
                .items
                .iter()
                .filter(|item| {
                    is_interest_item(
                        item,
                        pub_use_info,
                        Some((&src_namespace, rust_output_path_namespace)),
                    )
                })
                .filter_map(name_for_use_stmt)
                .collect::<HashSet<_>>();
            let (interest_items, remaining_items) = std::mem::take(&mut src_mod.items)
                .into_iter()
                .partition(|item| {
                    is_item_or_impl_interesting(
                        item,
                        &interest_names,
                        pub_use_info.rename.is_some(),
                    )
                });
            src_mod.items = remaining_items;
            interest_items
        } else {
            let interest_names = src_mod
                .items
                .iter()
                .filter(|item| is_interest_item(item, pub_use_info, None))
                .filter_map(name_for_use_stmt)
                .collect::<HashSet<_>>();
            src_mod
                .items
                .iter()
                .filter(|item| {
                    is_item_or_impl_interesting(
                        item,
                        &interest_names,
                        pub_use_info.rename.is_some(),
                    )
                })
                .cloned()
                .collect_vec()
        }
    } else {
        // Codecov seems to be buggy by saying this line is not covered (while lines above/below) are
        // frb-coverage:ignore-start
        log::debug!(
            "transform_module_by_pub_use_single skip `{pub_use_info:?}` since cannot find mod"
        );
        // frb-coverage:ignore-end
        vec![]
    };

    for item in &mut src_mod_interest_items {
        rename_item_for_use(item, pub_use_info);
    }

    let added_item_count = src_mod_interest_items.len();
    module.items.extend(src_mod_interest_items);
    module
        .item_contexts
        .extend(std::iter::repeat(None).take(added_item_count));

    Ok(())
}

fn rename_item_for_use(item: &mut syn::Item, pub_use_info: &PubUseInfo) {
    let Some(rename) = &pub_use_info.rename else {
        return;
    };
    let ident = syn::parse_str(rename).expect("re-export rename should be a valid identifier");
    match item {
        syn::Item::Struct(x) => x.ident = ident,
        syn::Item::Enum(x) => x.ident = ident,
        syn::Item::Type(x) => x.ident = ident,
        syn::Item::Fn(x) => x.sig.ident = ident,
        syn::Item::Trait(x) => x.ident = ident,
        syn::Item::Impl(x) => {
            if let syn::Type::Path(self_ty) = x.self_ty.as_mut() {
                if let Some(mut segment) = self_ty.path.segments.last().cloned() {
                    segment.ident = ident;
                    self_ty.qself = None;
                    self_ty.path.leading_colon = None;
                    self_ty.path.segments = std::iter::once(segment).collect();
                }
            }
        }
        _ => {}
    }
}

fn is_item_or_impl_interesting(
    item: &syn::Item,
    interest_names: &HashSet<String>,
    include_impls: bool,
) -> bool {
    name_for_use_stmt(item).is_some_and(|name| interest_names.contains(&name))
        || (include_impls
            && impl_target_name(item).is_some_and(|name| interest_names.contains(&name)))
}

fn impl_target_name(item: &syn::Item) -> Option<String> {
    let syn::Item::Impl(item_impl) = item else {
        return None;
    };
    let syn::Type::Path(self_ty) = item_impl.self_ty.as_ref() else {
        return None;
    };
    self_ty
        .path
        .segments
        .last()
        .map(|segment| segment.ident.to_string())
}

fn is_interest_item(
    item: &syn::Item,
    pub_use_info: &PubUseInfo,
    self_crate_namespaces: Option<(&Namespace, &Namespace)>,
) -> bool {
    let name_for_use_stmt = name_for_use_stmt(item).unwrap_or_else(|| "NOT_EXIST_NAME".to_owned());
    let is_visible =
        if let Some((declaration_namespace, rust_output_path_namespace)) = self_crate_namespaces {
            item_visibility(item).map_or(true, |visibility| {
                is_visibility_accessible_from(
                    visibility,
                    declaration_namespace,
                    rust_output_path_namespace,
                )
            })
        } else {
            is_item_public(item).unwrap_or(true)
        };
    pub_use_info.is_interest_name(&name_for_use_stmt) && is_visible && is_localized_definition(item)
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
    item_visibility(item).map(|vis| matches!(vis, syn::Visibility::Public(_)))
}

fn item_visibility(item: &syn::Item) -> Option<&syn::Visibility> {
    let vis = match item {
        syn::Item::Struct(x) => &x.vis,
        syn::Item::Enum(x) => &x.vis,
        syn::Item::Type(x) => &x.vis,
        syn::Item::Fn(x) => &x.vis,
        syn::Item::Trait(x) => &x.vis,
        _ => return None,
    };
    Some(vis)
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
    use crate::codegen::ir::hir::misc::visibility::HirVisibility;
    use crate::codegen::ir::hir::tree::crates::HirTreeCrate;
    use crate::codegen::ir::hir::tree::module::HirTreeModuleMeta;
    use crate::utils::crate_name::CrateName;

    #[test]
    pub fn test_parse_pub_use_from_item() {
        fn body(code: &str, expect: Vec<PubUseInfo>) {
            let item: syn::Item = syn::parse_str(code).unwrap();
            let actual = parse_pub_use_from_item(
                &item,
                &CrateName::self_crate().namespace(),
                &Namespace::new_self_crate("frb_generated".to_owned()),
            );
            assert_eq!(actual, expect);
        }

        body(
            "pub use one::two::*;",
            vec![PubUseInfo {
                namespace: Namespace::new_raw("one::two".to_owned()),
                name_filter: None,
                rename: None,
            }],
        );

        body(
            "pub use one::two::Three;",
            vec![PubUseInfo {
                namespace: Namespace::new_raw("one::two".to_owned()),
                name_filter: Some("Three".to_owned()),
                rename: None,
            }],
        );

        body(
            "pub(crate) use one::two::Three;",
            vec![PubUseInfo {
                namespace: Namespace::new_raw("one::two".to_owned()),
                name_filter: Some("Three".to_owned()),
                rename: None,
            }],
        );

        // https://github.com/fzyzcjy/flutter_rust_bridge/issues/2102#issuecomment-2179595124
        body(
            "pub use one::two::{x, y, z};",
            vec![
                PubUseInfo {
                    namespace: Namespace::new_raw("one::two".to_owned()),
                    name_filter: Some("x".to_owned()),
                    rename: None,
                },
                PubUseInfo {
                    namespace: Namespace::new_raw("one::two".to_owned()),
                    name_filter: Some("y".to_owned()),
                    rename: None,
                },
                PubUseInfo {
                    namespace: Namespace::new_raw("one::two".to_owned()),
                    name_filter: Some("z".to_owned()),
                    rename: None,
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
                    rename: None,
                },
                PubUseInfo {
                    namespace: Namespace::new_raw("one::two::u".to_owned()),
                    name_filter: Some("v".to_owned()),
                    rename: None,
                },
                PubUseInfo {
                    namespace: Namespace::new_raw("one::two::u".to_owned()),
                    name_filter: Some("w".to_owned()),
                    rename: None,
                },
            ],
        );

        body(
            "pub use one::two::Three as PublicThree;",
            vec![PubUseInfo {
                namespace: Namespace::new_raw("one::two".to_owned()),
                name_filter: Some("Three".to_owned()),
                rename: Some("PublicThree".to_owned()),
            }],
        );
    }

    /// Moves a self-crate re-exported definition out of its inaccessible module.
    #[test]
    pub fn test_transform_self_crate_pub_use() -> anyhow::Result<()> {
        let hidden_module = HirTreeModule {
            meta: HirTreeModuleMeta {
                parent_vis: vec![HirVisibility::Public],
                vis: HirVisibility::Inherited,
                namespace: Namespace::new_self_crate("hidden".to_owned()),
                is_accessible_from_rust_output: false,
            },
            modules: vec![],
            items: vec![
                syn::parse_str("pub(crate) struct Thing { pub value: String }")?,
                syn::parse_str("pub(crate) struct ThingExtra { pub value: Thing }")?,
            ],
            item_contexts: vec![None, None],
        };
        let root_module = HirTreeModule {
            meta: HirTreeModuleMeta {
                parent_vis: vec![],
                vis: HirVisibility::Public,
                namespace: CrateName::self_crate().namespace(),
                is_accessible_from_rust_output: true,
            },
            modules: vec![hidden_module],
            items: vec![syn::parse_str("pub(crate) use crate::hidden::ThingExtra;")?],
            item_contexts: vec![None],
        };
        let pack = HirTreePack {
            crates: vec![HirTreeCrate {
                name: CrateName::self_crate(),
                root_module,
            }],
        };

        let output = transform(pack, &Namespace::new_self_crate("frb_generated".to_owned()))?;
        let root = &output.crates[0].root_module;

        assert!(root
            .items
            .iter()
            .any(|item| name_for_use_stmt(item).as_deref() == Some("ThingExtra")));
        assert!(root.modules[0]
            .items
            .iter()
            .any(|item| name_for_use_stmt(item).as_deref() == Some("Thing")));
        assert!(!root.modules[0]
            .items
            .iter()
            .any(|item| name_for_use_stmt(item).as_deref() == Some("ThingExtra")));
        let moved_index = root
            .items
            .iter()
            .position(|item| name_for_use_stmt(item).as_deref() == Some("ThingExtra"))
            .unwrap();
        assert_eq!(
            root.item_contexts[moved_index]
                .as_ref()
                .unwrap()
                .declaration_namespace,
            Namespace::new_self_crate("hidden".to_owned()),
        );
        Ok(())
    }

    /// Moves a definition through an inaccessible re-export facade.
    #[test]
    fn test_transform_transitive_self_crate_pub_use() -> anyhow::Result<()> {
        let hidden_module = HirTreeModule {
            meta: HirTreeModuleMeta {
                parent_vis: vec![HirVisibility::Public],
                vis: HirVisibility::Inherited,
                namespace: Namespace::new_self_crate("hidden".to_owned()),
                is_accessible_from_rust_output: false,
            },
            modules: vec![],
            items: vec![syn::parse_str(
                "pub(crate) struct Thing { pub value: String }",
            )?],
            item_contexts: vec![None],
        };
        let facade_module = HirTreeModule {
            meta: HirTreeModuleMeta {
                parent_vis: vec![HirVisibility::Public],
                vis: HirVisibility::Inherited,
                namespace: Namespace::new_self_crate("facade".to_owned()),
                is_accessible_from_rust_output: false,
            },
            modules: vec![],
            items: vec![syn::parse_str("pub(crate) use super::hidden::Thing;")?],
            item_contexts: vec![None],
        };
        let root_module = HirTreeModule {
            meta: HirTreeModuleMeta {
                parent_vis: vec![],
                vis: HirVisibility::Public,
                namespace: CrateName::self_crate().namespace(),
                is_accessible_from_rust_output: true,
            },
            modules: vec![hidden_module, facade_module],
            items: vec![syn::parse_str("pub(crate) use facade::Thing;")?],
            item_contexts: vec![None],
        };
        let pack = HirTreePack {
            crates: vec![HirTreeCrate {
                name: CrateName::self_crate(),
                root_module,
            }],
        };

        let output = transform(pack, &Namespace::new_self_crate("frb_generated".to_owned()))?;
        let root = &output.crates[0].root_module;
        let moved_index = root
            .items
            .iter()
            .position(|item| name_for_use_stmt(item).as_deref() == Some("Thing"))
            .unwrap();

        assert_eq!(
            root.item_contexts[moved_index]
                .as_ref()
                .unwrap()
                .declaration_namespace,
            Namespace::new_self_crate("hidden".to_owned()),
        );
        assert!(!root.modules.iter().any(|module| {
            module
                .items
                .iter()
                .any(|item| name_for_use_stmt(item).as_deref() == Some("Thing"))
        }));
        Ok(())
    }

    /// Moves a renamed re-export under its publicly usable name.
    #[test]
    fn test_transform_renamed_self_crate_pub_use() -> anyhow::Result<()> {
        let hidden_module = HirTreeModule {
            meta: HirTreeModuleMeta {
                parent_vis: vec![HirVisibility::Public],
                vis: HirVisibility::Inherited,
                namespace: Namespace::new_self_crate("hidden".to_owned()),
                is_accessible_from_rust_output: false,
            },
            modules: vec![],
            items: vec![
                syn::parse_str("pub(crate) struct Inner { pub value: String }")?,
                syn::parse_str("impl Inner { pub fn value(&self) -> &str { &self.value } }")?,
            ],
            item_contexts: vec![None, None],
        };
        let root_module = HirTreeModule {
            meta: HirTreeModuleMeta {
                parent_vis: vec![],
                vis: HirVisibility::Public,
                namespace: CrateName::self_crate().namespace(),
                is_accessible_from_rust_output: true,
            },
            modules: vec![hidden_module],
            items: vec![syn::parse_str(
                "pub(crate) use crate::hidden::Inner as PublicInner;",
            )?],
            item_contexts: vec![None],
        };
        let pack = HirTreePack {
            crates: vec![HirTreeCrate {
                name: CrateName::self_crate(),
                root_module,
            }],
        };

        let output = transform(pack, &Namespace::new_self_crate("frb_generated".to_owned()))?;
        let root = &output.crates[0].root_module;

        assert!(root
            .items
            .iter()
            .any(|item| name_for_use_stmt(item).as_deref() == Some("PublicInner")));
        assert!(root
            .items
            .iter()
            .any(|item| impl_target_name(item).as_deref() == Some("PublicInner")));
        assert!(!root.modules[0].items.iter().any(|item| {
            name_for_use_stmt(item).as_deref() == Some("Inner")
                || impl_target_name(item).as_deref() == Some("Inner")
        }));
        Ok(())
    }

    /// Resolves self and parent re-export module paths from their declaration module.
    #[test]
    fn test_resolve_self_crate_pub_use_namespace() {
        let declaration = Namespace::new_self_crate("api::exports".to_owned());

        assert_eq!(
            resolve_pub_use_namespace(&Namespace::new_raw("self::hidden".to_owned()), &declaration,),
            Some(Namespace::new_self_crate("api::exports::hidden".to_owned(),)),
        );
        assert_eq!(
            resolve_pub_use_namespace(
                &Namespace::new_raw("super::hidden".to_owned()),
                &declaration,
            ),
            Some(Namespace::new_self_crate("api::hidden".to_owned())),
        );
    }
}
