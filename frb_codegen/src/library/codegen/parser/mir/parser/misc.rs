use crate::codegen::ir::hir::flat::struct_or_enum::HirFlatStructOrEnum;
use crate::codegen::ir::hir::misc::syn_item_struct_or_enum::SynItemStructOrEnum;
use crate::codegen::parser::mir::internal_config::RustInputNamespacePack;
use crate::utils::namespace::NamespacedName;
use itertools::Itertools;
use std::collections::HashMap;

pub(crate) fn extract_src_types_in_paths<I: SynItemStructOrEnum>(
    src_items: &HashMap<String, &HirFlatStructOrEnum<I>>,
    rust_input_namespace_pack: &RustInputNamespacePack,
) -> anyhow::Result<Vec<NamespacedName>> {
    Ok((src_items.iter())
        .filter_map(|(k, v)| {
            let namespace = &v.name.namespace;
            if rust_input_namespace_pack.is_interest(namespace) {
                Some(NamespacedName::new(namespace.to_owned(), k.to_owned()))
            } else {
                None
            }
        })
        .collect_vec())
}
