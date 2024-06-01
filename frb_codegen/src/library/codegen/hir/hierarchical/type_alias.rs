use crate::codegen::hir::hierarchical::struct_or_enum::serialize_syn;
use serde::Serialize;
use syn::Type;

#[derive(Clone, Debug, Serialize)]
pub struct TypeAlias {
    pub(super) ident: String,
    #[serde(serialize_with = "serialize_syn")]
    pub(super) target: Type,
}
