use crate::codegen::ir::hir::flat::function::{HirFlatFunction, HirFlatFunctionOwner};
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::flat::trait_impl::HirFlatTraitImpl;
use crate::codegen::ir::hir::misc::generation_source::HirGenerationSource;
use itertools::{concat, Itertools};

pub(crate) fn transform(mut pack: HirFlatPack) -> anyhow::Result<HirFlatPack> {
    for trait_impl in &pack.trait_impls {
        pack.functions.extend(compute_functions(trait_impl, &pack));
    }
    Ok(pack)
}

fn compute_functions(trait_impl: &HirFlatTraitImpl, pack: &HirFlatPack) -> Vec<HirFlatFunction> {
    (pack.functions.iter())
        .filter(|f| {
            if let HirFlatFunctionOwner::TraitDef { trait_def_name } = &f.owner {
                trait_def_name.name == trait_impl.trait_name
            } else {
                false
            }
        })
        .map(|f| HirFlatFunction {
            namespace: f.namespace.clone(), // TODO correct?
            owner: HirFlatFunctionOwner::StructOrEnum {
                impl_ty: trait_impl.impl_ty.clone(),
                trait_def_name: Some(trait_impl.trait_name.clone()),
            },
            sources: concat([
                f.sources.clone(),
                vec![HirGenerationSource::CopyFromTraitDef],
            ]),
            item_fn: f.item_fn.clone(),
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::ir::hir::misc::generation_source::HirGenerationSource;
    use crate::codegen::ir::hir::misc::item_fn::GeneralizedItemFn;
    use crate::utils::namespace::{Namespace, NamespacedName};
    use syn::parse_quote;

    /// Copies matching trait definitions into the implemented type.
    #[test]
    fn copies_only_matching_trait_functions() {
        let namespace = Namespace::new_raw("crate::api".to_owned());
        let mut pack = HirFlatPack::default();
        pack.trait_impls.push(HirFlatTraitImpl {
            trait_name: "Service".to_owned(),
            impl_ty: parse_quote!(Widget),
        });
        pack.functions.push(HirFlatFunction {
            namespace: namespace.clone(),
            owner: HirFlatFunctionOwner::TraitDef {
                trait_def_name: NamespacedName::new(namespace.clone(), "Service".to_owned()),
            },
            sources: vec![HirGenerationSource::Normal],
            item_fn: GeneralizedItemFn::TraitItemFn(parse_quote!(
                fn run(&self);
            )),
        });
        pack.functions.push(HirFlatFunction {
            namespace,
            owner: HirFlatFunctionOwner::TraitDef {
                trait_def_name: NamespacedName::new(
                    Namespace::new_raw("crate::api".to_owned()),
                    "Other".to_owned(),
                ),
            },
            sources: vec![],
            item_fn: GeneralizedItemFn::TraitItemFn(parse_quote!(
                fn skip(&self);
            )),
        });

        let transformed = transform(pack).unwrap();

        assert_eq!(transformed.functions.len(), 3);
        let copied = transformed.functions.last().unwrap();
        assert_eq!(copied.item_fn.name(), "run");
        assert_eq!(
            copied.sources,
            vec![
                HirGenerationSource::Normal,
                HirGenerationSource::CopyFromTraitDef
            ]
        );
        assert!(matches!(
            copied.owner,
            HirFlatFunctionOwner::StructOrEnum { ref trait_def_name, .. }
                if trait_def_name.as_deref() == Some("Service")
        ));
    }
}
