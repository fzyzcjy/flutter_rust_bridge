use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::flat::struct_or_enum::HirFlatStructOrEnum;

pub(crate) mod base;
pub(crate) mod third_party_override_merger;
pub(crate) mod trait_impl_merger;

pub(crate) fn transform(mut pack: HirFlatPack) -> anyhow::Result<HirFlatPack> {
    transform_component(&mut pack.functions);
    transform_component(&mut pack.structs);
    transform_component(&mut pack.enums);
    Ok(pack)
}

fn transform_component<T>(items: &mut Vec<T>) {
    TODO
}
