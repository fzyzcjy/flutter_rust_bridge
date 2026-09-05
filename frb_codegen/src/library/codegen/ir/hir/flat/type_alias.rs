use crate::codegen::ir::hir::flat::component::HirFlatComponent;
use crate::codegen::ir::hir::misc::serializers::serialize_syn;
use serde::Serialize;
use syn::Type;

#[derive(Clone, Debug, Serialize)]
pub struct HirFlatTypeAlias {
    pub(crate) ident: String,
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Uses the alias identifier as its sort key.
    #[test]
    fn sort_key_uses_alias_identifier() {
        let alias = HirFlatTypeAlias {
            ident: "ResultAlias".to_owned(),
            target: syn::parse_str("Result<T, Error>").unwrap(),
            type_params: vec!["T".to_owned()],
        };

        assert_eq!(alias.sort_key(), "ResultAlias");
    }
}
