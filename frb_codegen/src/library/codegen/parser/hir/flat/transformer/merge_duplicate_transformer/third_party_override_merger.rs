use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::struct_or_enum::{HirFlatEnum, HirFlatStruct};
use crate::codegen::ir::hir::flat::traits::HirFlatTrait;
use crate::codegen::ir::hir::misc::generation_source::HirGenerationSource;
use crate::codegen::ir::hir::misc::syn_item_struct_or_enum::SynItemStructOrEnum;
use crate::codegen::parser::hir::flat::transformer::merge_duplicate_transformer::base::BaseMerger;

pub(crate) struct ThirdPartyOverrideMerger;

impl BaseMerger for ThirdPartyOverrideMerger {
    fn merge_functions(
        &self,
        base: &HirFlatFunction,
        overrider: &HirFlatFunction,
    ) -> Option<HirFlatFunction> {
        merge_core(base, &overrider.sources, |ans| {
            (ans.item_fn.attrs_mut()).extend(overrider.item_fn.attrs().to_owned())
        })
    }

    fn merge_structs(
        &self,
        base: &HirFlatStruct,
        overrider: &HirFlatStruct,
    ) -> Option<HirFlatStruct> {
        merge_core(base, &overrider.sources, |ans| {
            ans.src.attrs_mut().extend(overrider.src.attrs().to_owned());
        })
    }

    fn merge_enums(&self, base: &HirFlatEnum, overrider: &HirFlatEnum) -> Option<HirFlatEnum> {
        merge_core(base, &overrider.sources, |ans| {
            ans.src.attrs_mut().extend(overrider.src.attrs().to_owned());
        })
    }

    fn merge_traits(&self, base: &HirFlatTrait, overrider: &HirFlatTrait) -> Option<HirFlatTrait> {
        merge_core(base, &overrider.sources, |ans| {
            ans.attrs.extend(overrider.attrs.clone());
        })
    }
}

fn merge_core<T: Clone>(
    base: &T,
    overrider_sources: &[HirGenerationSource],
    writer: impl Fn(&mut T),
) -> Option<T> {
    (overrider_sources.contains(&HirGenerationSource::MoveFromCrateThirdPartyFolder)).then(|| {
        let mut ans = base.to_owned();
        writer(&mut ans);
        ans
    })
}
