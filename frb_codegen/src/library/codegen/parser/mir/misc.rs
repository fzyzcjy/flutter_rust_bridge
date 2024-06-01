use crate::codegen::ir::hir::hierarchical::struct_or_enum::HirStructOrEnumWrapper;
use crate::codegen::ir::mir::namespace::NamespacedName;
use crate::codegen::parser::mir::internal_config::RustInputNamespacePack;
use itertools::Itertools;
use std::collections::HashMap;

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
