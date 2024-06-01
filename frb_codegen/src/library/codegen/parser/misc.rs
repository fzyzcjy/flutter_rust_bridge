use crate::codegen::hir::hierarchical::struct_or_enum::HirStructOrEnumWrapper;
use crate::codegen::ir::namespace::{Namespace, NamespacedName};
use crate::codegen::parser::internal_config::RustInputNamespacePack;
use crate::library::misc::consts::HANDLER_NAME;
use itertools::Itertools;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub(crate) fn parse_has_executor(source_rust_content: &str) -> bool {
    source_rust_content.contains(&format!("static {HANDLER_NAME}"))
}

pub(crate) fn extract_src_types_in_paths<T: HirStructOrEnumWrapper<I>, I>(
    src_items: &HashMap<String, &T>,
    rust_input_namespace_pack: &RustInputNamespacePack,
) -> anyhow::Result<Vec<NamespacedName>> {
    Ok((src_items.iter())
        .filter_map(|(k, v)| {
            let namespace = &v.inner().namespaced_name.namespace;
            if rust_input_namespace_pack.is_interest(namespace) {
                Some(NamespacedName::new(namespace.to_owned(), k.to_owned()))
            } else {
                None
            }
        })
        .collect_vec())
}
