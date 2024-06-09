use crate::codegen::ir::mir::ty::delegate::MirTypeDelegateProxyVariant;
use crate::codegen::ir::mir::ty::MirType;
use crate::utils::namespace::Namespace;

#[derive(Debug, Clone, serde::Serialize, PartialEq, Eq, Hash)]
pub(crate) struct IrEarlyGeneratorTraitDefInfo {
    pub original_ty: MirType,
    pub delegate_namespace: Namespace,
    pub variants: Vec<TODO>,
}
