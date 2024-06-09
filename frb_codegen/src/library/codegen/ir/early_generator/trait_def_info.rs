use crate::codegen::ir::mir::ty::delegate::MirTypeDelegateDynTraitVariant;
use crate::utils::namespace::{Namespace, NamespacedName};

#[derive(Debug, Clone, serde::Serialize, PartialEq, Eq, Hash)]
pub(crate) struct IrEarlyGeneratorTraitDefInfo {
    pub trait_def_name: NamespacedName,
    pub delegate_namespace: Namespace,
    pub variants: Vec<MirTypeDelegateDynTraitVariant>,
}
