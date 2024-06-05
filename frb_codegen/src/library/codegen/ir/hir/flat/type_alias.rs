use crate::codegen::ir::hir::hierarchical::struct_or_enum::serialize_syn;
use serde::Serialize;
use syn::Type;

#[derive(Clone, Debug, Serialize)]
pub struct HirFlatTypeAlias {
    pub(crate) ident: String,
    #[serde(serialize_with = "serialize_syn")]
    pub(crate) target: Type,
}
