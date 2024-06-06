use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::pack::HirFlatPack;

pub(crate) fn transform(mut pack: HirFlatPack) -> anyhow::Result<HirFlatPack> {
    for function in pack.functions.iter_mut() {
        transform_function(function)?;
    }
    Ok(pack)
}

fn transform_function(function: &mut HirFlatFunction) -> anyhow::Result<()> {
    if let Some(func_name_stripped) = function.item_fn.name().strip_prefix(FRB_OVERRIDE_PREFIX) {
        let attr_str = format!(r###"#[frb(name = "{}")]"###, func_name_stripped);
        let attr = syn::parse_str(&attr_str)?;
        function.item_fn.attrs_mut().push(attr);
    }
    Ok(())
}

const FRB_OVERRIDE_PREFIX: &str = "frb_override_";
