use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::struct_or_enum::{HirFlatEnum, HirFlatStruct};
use crate::codegen::ir::hir::misc::syn_item_struct_or_enum::SynItemStructOrEnum;
use crate::codegen::misc::THIRD_PARTY_DIR_NAME;
use crate::codegen::parser::hir::flat::transformer::merge_duplicate_transformer::base::BaseMerger;
use crate::utils::crate_name::CrateName;
use crate::utils::namespace::Namespace;

pub(crate) struct ThirdPartyOverrideMerger;

impl BaseMerger for ThirdPartyOverrideMerger {
    fn merge_functions(
        &self,
        base: &HirFlatFunction,
        overrider: &HirFlatFunction,
    ) -> Option<HirFlatFunction> {
        log::info!("hi merge_functions {} {}", serde_json::to_string(base).unwrap(), serde_json::to_string(overrider).unwrap());
        merge_core(base, &overrider.namespace, |ans| {
            (ans.item_fn.attrs_mut()).extend(overrider.item_fn.attrs().to_owned())
        })
    }

    fn merge_structs(
        &self,
        base: &HirFlatStruct,
        overrider: &HirFlatStruct,
    ) -> Option<HirFlatStruct> {
        merge_core(base, &overrider.name.namespace, |ans| {
            ans.src.attrs_mut().extend(overrider.src.attrs().to_owned());
        })
    }

    fn merge_enums(&self, base: &HirFlatEnum, overrider: &HirFlatEnum) -> Option<HirFlatEnum> {
        merge_core(base, &overrider.name.namespace, |ans| {
            ans.src.attrs_mut().extend(overrider.src.attrs().to_owned());
        })
    }
}

fn merge_core<T: Clone>(
    base: &T,
    overrider_namespace: &Namespace,
    writer: impl Fn(&mut T),
) -> Option<T> {
    is_module_third_party(overrider_namespace).then(|| {
        let mut ans = base.to_owned();
        writer(&mut ans);
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
