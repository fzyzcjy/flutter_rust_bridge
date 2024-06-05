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
    pub(crate) mirror: bool,
    #[derivative(Debug = "ignore")]
    #[serde(skip_serializing)]
    pub(crate) src: Item,
}

pub type HirFlatStruct = HirFlatStructOrEnum<ItemStruct>;
pub type HirFlatEnum = HirFlatStructOrEnum<ItemEnum>;
