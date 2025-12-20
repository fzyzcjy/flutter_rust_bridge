use crate::codegen::ir::hir::flat::component::HirFlatComponent;
use crate::codegen::ir::hir::misc::serializers::{serialize_option_syn, serialize_syn};
use serde::Serialize;
use syn::{Generics, Type};

#[derive(Clone, Debug, Serialize)]
pub struct HirFlatTypeAlias {
    pub(crate) ident: String,
    #[serde(serialize_with = "serialize_syn")]
    pub(crate) target: Type,
    #[serde(serialize_with = "serialize_option_syn")]
    pub(crate) generics: Option<Generics>,
}

impl HirFlatComponent<String> for HirFlatTypeAlias {
    fn sort_key(&self) -> String {
        self.ident.to_string()
    }
}
