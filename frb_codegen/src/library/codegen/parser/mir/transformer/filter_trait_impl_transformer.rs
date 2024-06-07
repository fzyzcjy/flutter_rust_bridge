use crate::codegen::ir::mir::pack::MirPack;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use std::collections::HashSet;

pub(crate) fn transform(mut pack: MirPack) -> anyhow::Result<MirPack> {
    let distinct_types = pack.distinct_types(None);
    let distinct_type_safe_idents = (distinct_types.iter())
        .map(|ty| ty.safe_ident())
        .collect::<HashSet<_>>();

    pack.trait_impls = (pack.trait_impls.drain(..))
        .filter(|item| distinct_type_safe_idents.contains(&item.impl_ty.safe_ident()))
        .collect();

    Ok(pack)
}
