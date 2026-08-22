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
        let attr_extra_str = format!(r###"#[frb(name = "{func_name_stripped}")]"###);
        let attr_extra = parse_attribute(&attr_extra_str)?;

        function.sources.push(HirGenerationSource::FromFrbOverride);
        function.item_fn.attrs_mut().push(attr_extra);
    }
    Ok(())
}

const FRB_OVERRIDE_PREFIX: &str = "frb_override_";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::ir::hir::flat::function::HirFlatFunctionOwner;
    use crate::codegen::ir::hir::misc::item_fn::GeneralizedItemFn;
    use crate::utils::namespace::Namespace;
    use syn::parse_quote;

    fn function(source: syn::ItemFn) -> HirFlatFunction {
        HirFlatFunction {
            namespace: Namespace::new_raw("crate::api".to_owned()),
            owner: HirFlatFunctionOwner::Function,
            sources: vec![],
            item_fn: GeneralizedItemFn::ItemFn(source),
        }
    }

    /// Adds the public name and provenance for override functions.
    #[test]
    fn transforms_override_function() {
        let mut parsed = function(parse_quote!(
            pub fn frb_override_run() {}
        ));
        transform_function(&mut parsed).unwrap();

        assert_eq!(parsed.sources, vec![HirGenerationSource::FromFrbOverride]);
        assert_eq!(parsed.name_for_dedup(), "run");
    }

    /// Leaves ordinary function metadata untouched.
    #[test]
    fn leaves_ordinary_function_untouched() {
        let mut parsed = function(parse_quote!(
            pub fn run() {}
        ));
        transform_function(&mut parsed).unwrap();

        assert!(parsed.sources.is_empty());
        assert!(parsed.item_fn.attrs().is_empty());
    }
}
