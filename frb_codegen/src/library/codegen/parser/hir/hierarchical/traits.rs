use crate::codegen::ir::hir::hierarchical::traits::HirTrait;
use crate::utils::namespace::Namespace;
use syn::ItemImpl;

pub(crate) fn parse_syn_item_trait(item_trait: &ItemImpl, namespace: &Namespace) -> HirTrait {
    HirTrait {
        namespace: namespace.clone(),
        item_trait: item_trait.clone(),
    }
}
