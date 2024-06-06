use crate::codegen::ir::hir::flat::component::HirFlatComponent;
use crate::codegen::ir::hir::flat::source::HirFlatGenerationSource;
use crate::codegen::ir::hir::misc::syn_item_struct_or_enum::SynItemStructOrEnum;
use crate::codegen::ir::hir::misc::visibility::HirVisibility;
use crate::utils::namespace::NamespacedName;
use derivative::Derivative;
use serde::Serialize;
use syn::{ItemEnum, ItemStruct};

#[derive(Clone, Derivative, Serialize)]
#[derivative(Debug)]
pub struct HirFlatStructOrEnum<Item: SynItemStructOrEnum> {
    pub(crate) name: NamespacedName,
    pub(crate) visibility: HirVisibility,
    pub(crate) source: HirFlatGenerationSource,
    pub(crate) mirror: bool,
    #[derivative(Debug = "ignore")]
    #[serde(skip_serializing)]
    pub(crate) src: Item,
}

impl<Item: SynItemStructOrEnum> HirFlatComponent<NamespacedName> for HirFlatStructOrEnum<Item> {
    fn sort_key(&self) -> NamespacedName {
        self.name.clone()
    }
}

pub type HirFlatStruct = HirFlatStructOrEnum<ItemStruct>;
pub type HirFlatEnum = HirFlatStructOrEnum<ItemEnum>;
