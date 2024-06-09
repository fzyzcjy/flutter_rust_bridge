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
