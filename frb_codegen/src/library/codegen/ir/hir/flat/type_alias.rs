use crate::codegen::ir::hir::flat::component::HirFlatComponent;
use crate::codegen::ir::hir::misc::serializers::serialize_syn;
use crate::utils::namespace::Namespace;
use serde::Serialize;
use syn::Type;

#[derive(Clone, Debug, Serialize)]
pub struct HirFlatTypeAlias {
    pub(crate) ident: String,
    #[serde(skip_serializing)]
    pub(crate) namespace: Namespace,
    #[serde(skip_serializing)]
    pub(crate) declaration_namespace: Namespace,
    #[serde(skip_serializing)]
    pub(crate) imports: Vec<syn::ItemUse>,
    #[serde(serialize_with = "serialize_syn")]
    pub(crate) target: Type,
    /// Type parameter names of a generic alias, e.g. `["T"]` for
    /// `type AppResult<T> = Result<T, AppError>`. Empty for non-generic aliases.
    pub(crate) type_params: Vec<String>,
}

impl HirFlatComponent<String> for HirFlatTypeAlias {
    fn sort_key(&self) -> String {
        self.ident.to_string()
    }
}
