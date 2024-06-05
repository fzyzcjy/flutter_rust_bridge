use crate::codegen::ir::hir::misc::serializers::serialize_syn;
use quote::ToTokens;
use serde::{Serialize, Serializer};
use syn::Type;

#[derive(Clone, Debug, Serialize)]
pub struct HirFlatTypeAlias {
    pub(crate) ident: String,
    #[serde(serialize_with = "serialize_syn")]
    pub(crate) target: Type,
}
