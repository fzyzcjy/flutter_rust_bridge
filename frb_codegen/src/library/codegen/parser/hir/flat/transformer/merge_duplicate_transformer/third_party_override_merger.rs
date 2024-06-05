use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::struct_or_enum::HirFlatStructOrEnum;
use crate::codegen::ir::hir::hierarchical::function::HirFlatFunction;
use crate::codegen::ir::hir::hierarchical::module::HirModule;
use crate::codegen::ir::hir::hierarchical::pack::HirPack;
use crate::codegen::ir::hir::hierarchical::struct_or_enum::HirFlatStructOrEnum;
use crate::codegen::ir::hir::hierarchical::syn_item_struct_or_enum::SynItemStructOrEnum;
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
        base: HirFlatFunction,
        overrider: HirFlatFunction,
    ) -> Option<HirFlatFunction> {
        is_module_third_party(&overrider.namespace).then(|| {
            TODO;
        })
    }

    fn merge_struct_or_enums<Item: SynItemStructOrEnum>(
        &self,
        base: HirFlatStructOrEnum<Item>,
        overrider: HirFlatStructOrEnum<Item>,
    ) -> Option<HirFlatStructOrEnum<Item>> {
        is_module_third_party(&overrider.name.namespace).then(|| {
            TODO;
        })
    }
}

fn is_module_third_party(namespace: &Namespace) -> bool {
    Namespace::new(vec![
        CrateName::SELF_CRATE.to_owned(),
        THIRD_PARTY_DIR_NAME.to_owned(),
    ])
    .is_prefix_of(namespace)
}

fn transform_module_content_functions(
    target: &mut [HirFlatFunction],
    src_content_functions: Vec<HirFlatFunction>,
) -> anyhow::Result<()> {
    transform_module_content_general_vec(
        target,
        src_content_functions,
        |x| x.owner_and_name(),
        |target, src| {
            (target.item_fn.attrs_mut()).extend(src.item_fn.attrs().to_owned());
        },
    )
}

fn transform_module_content_struct_or_enums<Item: SynItemStructOrEnum>(
    target: &mut [HirFlatStructOrEnum<Item>],
    src_content_struct_or_enums: Vec<HirFlatStructOrEnum<Item>>,
) -> anyhow::Result<()> {
    transform_module_content_general_vec(
        target,
        src_content_struct_or_enums,
        |x| x.name.name.clone(),
        |target, src| {
            target.src.attrs_mut().extend(src.src.attrs().to_owned());
        },
    )
}
