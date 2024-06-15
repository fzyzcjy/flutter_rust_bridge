use crate::codegen::ir::mir::ty::delegate::MirTypeDelegateProxyVariant;
use crate::codegen::ir::mir::ty::MirType;
use crate::utils::namespace::Namespace;

#[derive(Debug, Clone, serde::Serialize, PartialEq, Eq, Hash)]
pub(crate) struct IrEarlyGeneratorProxiedType {
    pub original_ty: MirType,
    pub proxy_enum_namespace: Namespace,
    pub variants: Vec<MirTypeDelegateProxyVariant>,
}
