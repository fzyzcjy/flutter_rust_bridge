use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::struct_or_enum::HirFlatStructOrEnum;
use crate::codegen::misc::THIRD_PARTY_DIR_NAME;
use crate::codegen::parser::hir::flat::transformer::merge_duplicate_transformer::base::BaseMerger;
use crate::utils::crate_name::CrateName;
use crate::utils::namespace::Namespace;
use itertools::Itertools;
use std::fmt::Debug;

pub(crate) struct ThirdPartyOverrideMerger;

impl BaseMerger for ThirdPartyOverrideMerger {
    fn merge_functions(
        &self,
        base: &HirFlatFunction,
        overrider: &HirFlatFunction,
    ) -> Option<HirFlatFunction> {
        merge_core(base, overrider, &overrider.namespace, |ans| {
            (ans.item_fn.attrs_mut()).extend(overrider.item_fn.attrs().to_owned())
        })
    }

    fn merge_struct_or_enums<Item: SynItemStructOrEnum>(
        &self,
        base: &HirFlatStructOrEnum<Item>,
        overrider: &HirFlatStructOrEnum<Item>,
    ) -> Option<HirFlatStructOrEnum<Item>> {
        merge_core(base, overrider, &overrider.name.namespace, |ans| {
            ans.src.attrs_mut().extend(overrider.src.attrs().to_owned());
        })
    }
}

fn merge_core<T>(
    base: &T,
    overrider: &T,
    overrider_namespace: &Namespace,
    writer: impl Fn(&mut T),
) -> Option<T> {
    is_module_third_party(&namespace).then(|| {
        let mut ans = base.to_owned();
        writer(ans);
        ans
    })
}

fn is_module_third_party(namespace: &Namespace) -> bool {
    Namespace::new(vec![
        CrateName::SELF_CRATE.to_owned(),
        THIRD_PARTY_DIR_NAME.to_owned(),
    ])
    .is_prefix_of(namespace)
}
