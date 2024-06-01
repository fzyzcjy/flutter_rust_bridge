use crate::codegen::hir::hierarchical::struct_or_enum::StructOrEnumWrapper;
use crate::codegen::ir::namespace::{Namespace, NamespacedName};
use crate::library::misc::consts::HANDLER_NAME;
use itertools::Itertools;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub(crate) fn parse_has_executor(source_rust_content: &str) -> bool {
    source_rust_content.contains(&format!("static {HANDLER_NAME}"))
}

pub(crate) fn extract_src_types_in_paths<T: StructOrEnumWrapper<I>, I>(
    src_items: &HashMap<String, &T>,
    rust_input_namespaces: &[Namespace],
) -> anyhow::Result<Vec<NamespacedName>> {
    Ok((src_items.iter())
        .filter_map(|(k, v)| {
            let namespace = v.inner().namespace();
            if rust_input_namespaces.contains(&namespace) {
                Some(NamespacedName::new(namespace, k.to_owned()))
            } else {
                None
            }
        })
        .collect_vec())
}
