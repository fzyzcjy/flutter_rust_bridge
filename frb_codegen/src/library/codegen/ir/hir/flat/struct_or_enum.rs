use crate::codegen::ir::hir::flat::component::HirFlatComponent;
use crate::codegen::ir::hir::misc::generation_source::HirGenerationSource;
use crate::codegen::ir::hir::misc::syn_item_struct_or_enum::SynItemStructOrEnum;
use crate::codegen::ir::hir::misc::visibility::HirVisibility;
use crate::utils::namespace::NamespacedName;
use derivative::Derivative;
use serde::Serialize;
use syn::{ItemEnum, ItemStruct};

// This is surely used, but not counted by coverage tools
// frb-coverage:ignore-start
#[derive(Clone, Derivative, Serialize)]
#[derivative(Debug)]
pub struct HirFlatStructOrEnum<Item: SynItemStructOrEnum> {
    pub(crate) name: NamespacedName,
    pub(crate) visibility: HirVisibility,
    pub(crate) sources: Vec<HirGenerationSource>,
    pub(crate) mirror: bool,
    #[derivative(Debug = "ignore")]
    #[serde(skip_serializing)]
    pub(crate) src: Item,
}
// frb-coverage:ignore-end

impl<Item: SynItemStructOrEnum> HirFlatComponent<NamespacedName> for HirFlatStructOrEnum<Item> {
    fn sort_key(&self) -> NamespacedName {
        self.name.clone()
    }
}

pub type HirFlatStruct = HirFlatStructOrEnum<ItemStruct>;
pub type HirFlatEnum = HirFlatStructOrEnum<ItemEnum>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::ir::hir::misc::visibility::HirVisibility;
    use crate::utils::namespace::Namespace;

    /// Uses the namespaced type name as the sort key.
    #[test]
    fn sort_key_uses_namespaced_type_name() {
        let item: HirFlatStruct = HirFlatStructOrEnum {
            name: NamespacedName::new(
                Namespace::new_raw("crate::models".to_owned()),
                "Widget".to_owned(),
            ),
            visibility: HirVisibility::Public,
            sources: vec![],
            mirror: false,
            src: syn::parse_str("struct Widget;").unwrap(),
        };

        assert_eq!(item.sort_key().rust_style(), "crate::models::Widget");
    }
}
