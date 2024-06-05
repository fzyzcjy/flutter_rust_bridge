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
    fn merge_functions(&self, items: Vec<HirFlatFunction>) -> Vec<HirFlatFunction> {
        todo!()
    }

    fn merge_struct_or_enums<Item: SynItemStructOrEnum>(
        &self,
        items: Vec<HirFlatStructOrEnum<Item>>,
    ) -> Vec<HirFlatStructOrEnum<Item>> {
        todo!()
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

fn transform_module_content_general_vec<T: Debug, K: Eq + Debug>(
    target_items: &mut [T],
    src_items: Vec<T>,
    key: impl Fn(&T) -> K,
    write: impl Fn(&mut T, T),
) -> anyhow::Result<()> {
    for src_item in src_items {
        let src_key = key(&src_item);

        let interest_target_items = target_items
            .iter_mut()
            .filter(|x| key(x) == src_key)
            .collect_vec();
        if interest_target_items.len() != 1 {
            log::warn!(
                "transform_module_content_attrable skip src_key={src_key:?}, \
                since the number of corresponding target items is not one (indeed is {}). \
                src_item={src_item:?}",
                interest_target_items.len(),
            );
            continue;
        }
        let target_item = interest_target_items.into_iter().next().unwrap();

        write(target_item, src_item);
    }
    Ok(())
}
