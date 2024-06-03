use crate::codegen::ir::hir::hierarchical::traits::{HirTrait, HirTraitImpl};
use crate::utils::namespace::Namespace;
use syn::ItemImpl;
use syn::ItemTrait;

pub(crate) fn parse_syn_item_trait(item_trait: &ItemTrait, namespace: &Namespace) -> HirTrait {
    HirTrait {
        namespace: namespace.clone(),
        item_trait: item_trait.clone(),
    }
}

pub(crate) fn parse_trait_impl(item_impl: &ItemImpl, namespace: &Namespace) -> HirTraitImpl {
    HirTraitImpl {
        namespace: namespace.clone(),
        item_impl: item_impl.clone(),
    }
}
