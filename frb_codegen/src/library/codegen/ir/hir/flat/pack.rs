use crate::codegen::ir::hir::flat::component::HirFlatComponent;
use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::struct_or_enum::{HirFlatEnum, HirFlatStruct};
use crate::codegen::ir::hir::flat::trait_impl::HirFlatTraitImpl;
use crate::codegen::ir::hir::flat::traits::HirFlatTrait;
use crate::codegen::ir::hir::flat::type_alias::HirFlatTypeAlias;
use crate::codegen::ir::misc::skip::IrSkip;
use crate::utils::basic_code::general_code::GeneralDartCode;
use crate::utils::namespace::NamespacedName;
use crate::utils::usage_warner::UsageWarner;

#[derive(Debug, Clone, Default, serde::Serialize)]
pub(crate) struct HirFlatPack {
    pub functions: Vec<UsageWarner<HirFlatFunction>>,
    pub enums: Vec<UsageWarner<HirFlatEnum>>,
    pub structs: Vec<UsageWarner<HirFlatStruct>>,
    pub traits: Vec<UsageWarner<HirFlatTrait>>,
    pub trait_impls: Vec<UsageWarner<HirFlatTraitImpl>>,
    pub types: Vec<UsageWarner<HirFlatTypeAlias>>,
    pub existing_handler: Option<NamespacedName>,
    pub extra_rust_output_code: String,
    pub extra_dart_output_code: GeneralDartCode,
    pub skips: Vec<IrSkip>,
}

impl HirFlatPack {
    pub(crate) fn visit_components_mut(&mut self, visitor: impl HirFlatPackComponentVisitor) {
        visitor.visit(&mut self.functions);
        visitor.visit(&mut self.enums);
        visitor.visit(&mut self.structs);
        visitor.visit(&mut self.traits);
        visitor.visit(&mut self.trait_impls);
        visitor.visit(&mut self.types);
    }
}

pub(crate) trait HirFlatPackComponentVisitor {
    fn visit<SK: Ord, T: HirFlatComponent<SK>>(&self, items: &mut Vec<UsageWarner<T>>);
}
