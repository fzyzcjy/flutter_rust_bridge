use crate::codegen::ir::mir::pack::MirPack;
use crate::codegen::ir::mir::ty::delegate::{MirTypeDelegate, MirTypeDelegateDynTrait};
use crate::codegen::ir::mir::ty::MirType;
use crate::if_then_some;
use itertools::Itertools;

pub(crate) fn transform(mut pack: MirPack) -> anyhow::Result<MirPack> {
    let distinct_types = pack.distinct_types(None);

    let ty_dyn_traits = (distinct_types.iter())
        .filter_map(
            |ty| if_then_some!(let MirType::Delegate(MirTypeDelegate::DynTrait(ty)) = ty , ty.clone()),
        )
        .unique_by(|ty| ty.safe_ident())
        .collect_vec();

    for ty_dyn_trait in &ty_dyn_traits {
        handle_ty_dyn_trait(&mut pack, ty_dyn_trait)?;
    }

    Ok(pack)
}

fn handle_ty_dyn_trait(
    pack: &mut MirPack,
    ty_dyn_trait: &MirTypeDelegateDynTrait,
) -> anyhow::Result<()> {
    let interest_impl_types = (pack.trait_impls.iter())
        .filter(|item| item.trait_ty.name == ty_dyn_trait.trait_def_name)
        .map(|item| item.impl_ty.clone())
        .collect_vec();

    todo!()
}
