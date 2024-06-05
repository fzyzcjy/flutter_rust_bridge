use crate::codegen::ir::hir::flat::pack::HirFlatPack;

pub(crate) mod third_party_override_merger;
pub(crate) mod trait_impl_merger;

pub(crate) fn transform(mut pack: HirFlatPack) -> anyhow::Result<HirFlatPack> {
    TODO;
    Ok(pack)
}
