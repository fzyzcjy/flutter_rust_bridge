use crate::utils::namespace::Namespace;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub(crate) struct HirFlatConstant {
    pub(crate) namespace: Namespace,
    pub(crate) item_const: syn::ItemConst,
}
