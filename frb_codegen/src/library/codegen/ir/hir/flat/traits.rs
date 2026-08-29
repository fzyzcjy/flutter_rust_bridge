use crate::codegen::ir::hir::flat::component::HirFlatComponent;
use crate::codegen::ir::hir::misc::generation_source::HirGenerationSource;
use crate::codegen::ir::hir::misc::serializers::serialize_vec_syn;
use crate::utils::namespace::NamespacedName;

#[derive(Clone, serde::Serialize, Debug)]
pub struct HirFlatTrait {
    pub(crate) name: NamespacedName,
    #[serde(serialize_with = "serialize_vec_syn")]
    pub(crate) attrs: Vec<syn::Attribute>,
    pub(crate) sources: Vec<HirGenerationSource>,
}

impl HirFlatComponent<NamespacedName> for HirFlatTrait {
    fn sort_key(&self) -> NamespacedName {
        self.name.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::namespace::Namespace;

    /// Preserves namespace and trait name in the sort key.
    #[test]
    fn sort_key_preserves_namespace_and_name_ordering() {
        let item = HirFlatTrait {
            name: NamespacedName::new(
                Namespace::new_raw("crate::api".to_owned()),
                "Service".to_owned(),
            ),
            attrs: vec![],
            sources: vec![],
        };

        assert_eq!(item.sort_key().rust_style(), "crate::api::Service");
    }
}
