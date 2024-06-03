use serde::Serialize;
use syn::ItemTrait;
use crate::utils::namespace::Namespace;

#[derive(Debug, Clone, Serialize)]
pub(crate) struct HirTrait {
    pub(crate) namespace: Namespace,
    #[serde(skip_serializing)]
    pub(crate) item_trait: ItemTrait,
}
