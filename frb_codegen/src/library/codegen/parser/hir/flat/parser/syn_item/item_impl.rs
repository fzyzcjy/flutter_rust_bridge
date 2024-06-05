use crate::codegen::ir::hir::hierarchical::struct_or_enum::HirFlatStruct;
use crate::utils::namespace::Namespace;
use syn::{ItemImpl, ItemStruct};

pub(crate) fn parse_syn_item_impl(item: &ItemImpl) -> anyhow::Result<TODO> {
    if item_impl.trait_.is_some() {
        (pack.trait_impls).push(parse_trait_impl(item_impl, namespace));
    } else {
        (pack.functions).extend(parse_syn_item_impl(item_impl, namespace, None));
    }
}
