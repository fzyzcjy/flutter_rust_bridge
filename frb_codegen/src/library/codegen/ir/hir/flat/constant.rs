use crate::codegen::ir::hir::flat::component::HirFlatComponent;
use crate::codegen::ir::hir::misc::serializers::serialize_syn;
use crate::utils::namespace::Namespace;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub(crate) struct HirFlatConstant {
    pub(crate) namespace: Namespace,
    #[serde(serialize_with = "serialize_syn")]
    pub(crate) item_const: syn::ItemConst,
}

impl HirFlatComponent<String> for HirFlatConstant {
    fn sort_key(&self) -> String {
        self.item_const.ident.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Uses the constant identifier as its sort key.
    #[test]
    fn sort_key_uses_constant_identifier() {
        let item_const: syn::ItemConst = syn::parse_str("const ANSWER: u8 = 42;").unwrap();
        let constant = HirFlatConstant {
            namespace: Namespace::new_raw("crate::values".to_owned()),
            item_const,
        };

        assert_eq!(constant.sort_key(), "ANSWER");
    }
}
