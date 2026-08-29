use crate::codegen::ir::hir::flat::function::{HirFlatFunction, HirFlatFunctionOwner};
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::misc::skip::{IrSkip, IrSkipReason, IrValueOrSkip};
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
use crate::utils::namespace::NamespacedName;
use itertools::Itertools;
use std::collections::HashSet;

pub(crate) fn transform(mut pack: HirFlatPack) -> anyhow::Result<HirFlatPack> {
    let good_trait_names: HashSet<String> =
        (pack.traits.iter().map(|t| t.name.name.clone())).collect();

    let (funcs, skips) = IrValueOrSkip::split(
        (pack.functions.drain(..))
            .map(|f| {
                if should_retain(&f, &good_trait_names) {
                    IrValueOrSkip::Value(f)
                } else {
                    IrValueOrSkip::Skip(IrSkip {
                        name: NamespacedName::new(f.namespace.clone(), f.item_fn.name()),
                        reason: IrSkipReason::IgnoreBecauseNotDefinedTrait,
                    })
                }
            })
            .collect_vec(),
    );
    pack.functions = funcs;
    pack.skips.extend(skips);

    Ok(pack)
}

fn should_retain(f: &HirFlatFunction, good_trait_names: &HashSet<String>) -> bool {
    if let HirFlatFunctionOwner::StructOrEnum {
        trait_def_name: Some(trait_def_name),
        ..
    } = &f.owner
    {
        good_trait_names.contains(trait_def_name)
            || WHITELIST_TRAIT_NAMES.contains(&&**trait_def_name)
            || has_frb_attributes(f)
    } else {
        true
    }
}

// https://github.com/fzyzcjy/flutter_rust_bridge/issues/2103#issuecomment-2178061684
fn has_frb_attributes(f: &HirFlatFunction) -> bool {
    let attrs = FrbAttributes::parse(f.item_fn.attrs()).unwrap();
    !attrs.is_empty()
}

pub(crate) const WHITELIST_TRAIT_NAMES: [&str; 1] = ["Default"];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::ir::hir::flat::function::HirFlatFunctionOwner;
    use crate::codegen::ir::hir::misc::generation_source::HirGenerationSource;
    use crate::codegen::ir::hir::misc::item_fn::GeneralizedItemFn;
    use crate::utils::namespace::Namespace;

    fn trait_function(trait_name: &str, attributes: &str) -> HirFlatFunction {
        HirFlatFunction {
            namespace: Namespace::default(),
            owner: HirFlatFunctionOwner::StructOrEnum {
                impl_ty: syn::parse_str("Example").unwrap(),
                trait_def_name: Some(trait_name.to_owned()),
            },
            sources: vec![HirGenerationSource::Normal],
            item_fn: GeneralizedItemFn::ItemFn(
                syn::parse_str(&format!("{attributes} fn method() {{}}")).unwrap(),
            ),
        }
    }

    /// Retains methods whose defining trait is present in the parsed trait set.
    #[test]
    fn retains_method_for_defined_trait() {
        let function = trait_function("Defined", "");

        assert!(should_retain(
            &function,
            &HashSet::from(["Defined".to_owned()])
        ));
    }

    /// Retains whitelisted and explicitly annotated methods without their trait definition.
    #[test]
    fn retains_whitelisted_or_annotated_method_without_trait_definition() {
        let whitelisted = trait_function("Default", "");
        let annotated = trait_function("Missing", "#[frb(ignore)]");

        assert!(should_retain(&whitelisted, &HashSet::new()));
        assert!(should_retain(&annotated, &HashSet::new()));
    }

    /// Removes an unannotated method whose trait definition is absent.
    #[test]
    fn removes_method_for_undefined_trait_without_attribute() {
        let function = trait_function("Missing", "");

        assert!(!should_retain(&function, &HashSet::new()));
    }
}
