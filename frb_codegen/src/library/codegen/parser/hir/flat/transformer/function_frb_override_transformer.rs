use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::pack::HirFlatPack;

pub(crate) fn transform(mut pack: HirFlatPack) -> anyhow::Result<HirFlatPack> {
    for function in pack.functions.iter_mut() {
        transform_function(function);
    }
    Ok(pack)
}

fn transform_function(function: &mut HirFlatFunction) {
    if let Some(func_name_stripped) = function.item_fn.name().strip_prefix(FRB_OVERRIDE_PREFIX) {
        TODO
    }
}

const FRB_OVERRIDE_PREFIX: &str = "frb_override_";
