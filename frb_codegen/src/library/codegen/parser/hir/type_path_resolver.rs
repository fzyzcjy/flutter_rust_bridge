use crate::utils::namespace::{Namespace, NamespacedName};
use syn::{ItemUse, UseTree};

pub(crate) fn type_path_candidates(
    path: &syn::Path,
    initiated_namespace: &Namespace,
    imports: &[ItemUse],
) -> Vec<NamespacedName> {
    let segments = path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>();
    if segments.len() == 1 {
        let mut output = vec![NamespacedName::new(
            initiated_namespace.clone(),
            segments[0].clone(),
        )];
        for item_use in imports {
            collect_import_targets(
                &item_use.tree,
                &[],
                &segments[0],
                initiated_namespace,
                &mut output,
            );
        }
        output.sort();
        output.dedup();
        return output;
    }

    let type_namespace_segments = &segments[..segments.len() - 1];
    let mut output = vec![];
    if let Some(namespace) =
        resolve_relative_namespace(type_namespace_segments, initiated_namespace)
    {
        output.push(NamespacedName::new(
            namespace,
            segments.last().unwrap().clone(),
        ));
    }
    output.push(NamespacedName::new(
        Namespace::new(type_namespace_segments.to_vec()),
        segments.last().unwrap().clone(),
    ));
    let mut imported_modules = vec![];
    for item_use in imports {
        collect_import_targets(
            &item_use.tree,
            &[],
            &type_namespace_segments[0],
            initiated_namespace,
            &mut imported_modules,
        );
    }
    for imported_module in imported_modules {
        let mut namespace = imported_module.namespace.join(&imported_module.name);
        for segment in &type_namespace_segments[1..] {
            namespace = namespace.join(segment);
        }
        output.push(NamespacedName::new(
            namespace,
            segments.last().unwrap().clone(),
        ));
    }
    output.sort();
    output.dedup();
    output
}

fn collect_import_targets(
    tree: &UseTree,
    prefix: &[String],
    local_name: &str,
    initiated_namespace: &Namespace,
    output: &mut Vec<NamespacedName>,
) {
    match tree {
        UseTree::Path(inner) => {
            let mut child_prefix = prefix.to_vec();
            child_prefix.push(inner.ident.to_string());
            collect_import_targets(
                &inner.tree,
                &child_prefix,
                local_name,
                initiated_namespace,
                output,
            )
        }
        UseTree::Name(inner) => {
            if inner.ident == local_name {
                push_import_targets(prefix, inner.ident.to_string(), initiated_namespace, output);
            }
        }
        UseTree::Rename(inner) => {
            if inner.rename == local_name {
                push_import_targets(prefix, inner.ident.to_string(), initiated_namespace, output);
            }
        }
        UseTree::Glob(_) => {
            push_import_targets(prefix, local_name.to_owned(), initiated_namespace, output);
        }
        UseTree::Group(inner) => {
            for tree in &inner.items {
                collect_import_targets(tree, prefix, local_name, initiated_namespace, output);
            }
        }
    }
}

fn push_import_targets(
    prefix: &[String],
    item_name: String,
    initiated_namespace: &Namespace,
    output: &mut Vec<NamespacedName>,
) {
    if let Some(namespace) = resolve_relative_namespace(prefix, initiated_namespace) {
        output.push(NamespacedName::new(namespace, item_name.clone()));
    }
    output.push(NamespacedName::new(
        Namespace::new(prefix.to_vec()),
        item_name,
    ));
}

fn resolve_relative_namespace(
    type_namespace_segments: &[String],
    initiated_namespace: &Namespace,
) -> Option<Namespace> {
    let mut output = initiated_namespace
        .path()
        .into_iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    let mut index = 0;

    match type_namespace_segments.first().map(String::as_str) {
        Some("crate") => {
            output = vec!["crate".to_owned()];
            index = 1;
        }
        Some("self") => index = 1,
        Some("super") => {
            while type_namespace_segments.get(index).map(String::as_str) == Some("super") {
                output.pop()?;
                index += 1;
            }
        }
        _ => {}
    }

    output.extend(type_namespace_segments[index..].iter().cloned());
    Some(Namespace::new(output))
}
