use crate::codegen::ir::mir::ty::MirType;
use crate::utils::namespace::Namespace;

#[derive(Debug, Clone, serde::Serialize, PartialEq, Eq, Hash)]
pub(crate) struct IrEarlyGeneratorProxiedType {
    pub proxy_enum_namespace: Namespace,
    pub original_ty: MirType,
}
