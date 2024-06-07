use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::misc::generation_source::HirGenerationSource;
use crate::utils::syn_utils::parse_attribute;

pub(crate) fn transform(mut pack: HirFlatPack) -> anyhow::Result<HirFlatPack> {
    for function in pack.functions.iter_mut() {
        transform_function(function)?;
    }
    Ok(pack)
}

fn transform_function(function: &mut HirFlatFunction) -> anyhow::Result<()> {
    if let Some(func_name_stripped) = function.item_fn.name().strip_prefix(FRB_OVERRIDE_PREFIX) {
        let attr_extra_str = format!(r###"#[frb(name = "{}")]"###, func_name_stripped);
        let attr_extra = parse_attribute(&attr_extra_str)?;

        function.sources.push(HirGenerationSource::FromFrbOverride);
        function.item_fn.attrs_mut().push(attr_extra);
    }
    Ok(())
}

const FRB_OVERRIDE_PREFIX: &str = "frb_override_";
