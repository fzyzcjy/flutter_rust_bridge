use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::flat::struct_or_enum::HirFlatStructOrEnum;

pub(crate) mod third_party_override_merger;
pub(crate) mod trait_impl_merger;

pub(crate) fn transform(mut pack: HirFlatPack) -> anyhow::Result<HirFlatPack> {
    Ok(pack)
}

trait BaseMerger {
    fn merge_functions(&self, items: Vec<HirFlatFunction>) -> Vec<HirFlatFunction>;

    fn merge_struct_or_enums<Item: SynItemStructOrEnum>(
        &self,
        items: Vec<HirFlatStructOrEnum<Item>>,
    ) -> Vec<HirFlatStructOrEnum<Item>>;
}

struct CombinedMerger(pub Vec<dyn BaseMerger>);

impl BaseMerger for CombinedMerger {
    fn merge_functions(&self, mut items: Vec<HirFlatFunction>) -> Vec<HirFlatFunction> {
        for merger in &self.0 {
            items = merger.merge_functions(items);
        }
        items
    }

    fn merge_struct_or_enums<Item: SynItemStructOrEnum>(
        &self,
        items: Vec<HirFlatStructOrEnum<Item>>,
    ) -> Vec<HirFlatStructOrEnum<Item>> {
        for merger in &self.0 {
            items = merger.merge_struct_or_enums(items);
        }
        items
    }
}
