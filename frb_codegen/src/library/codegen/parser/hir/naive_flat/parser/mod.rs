use crate::codegen::ir::hir::misc::generation_source::HirGenerationSource;
use crate::codegen::ir::hir::naive_flat::item::{HirNaiveFlatItem, HirNaiveFlatItemMeta};
use crate::codegen::ir::hir::naive_flat::pack::HirNaiveFlatPack;
use crate::codegen::ir::hir::tree::module::HirTreeModule;
use crate::codegen::ir::hir::tree::pack::HirTreePack;

pub(crate) fn parse(pack: HirTreePack) -> anyhow::Result<HirNaiveFlatPack> {
    let mut items = vec![];
    for hir_crate in pack.crates {
        flatten_module(hir_crate.root_module, &mut items);
    }
    Ok(HirNaiveFlatPack { items })
}

fn flatten_module(module: HirTreeModule, target: &mut Vec<HirNaiveFlatItem>) {
    target.extend(module.items.into_iter().map(|item| HirNaiveFlatItem {
        meta: HirNaiveFlatItemMeta {
            namespace: module.meta.namespace.clone(),
            sources: vec![HirGenerationSource::Normal],
            is_module_public: module.meta.is_public(),
        },
        item,
    }));

    for child_module in module.modules {
        flatten_module(child_module, target);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::ir::hir::misc::visibility::HirVisibility;
    use crate::codegen::ir::hir::tree::crates::HirTreeCrate;
    use crate::codegen::ir::hir::tree::module::HirTreeModuleMeta;
    use crate::utils::crate_name::CrateName;
    use crate::utils::namespace::Namespace;
    use syn::parse_quote;

    fn module(
        namespace: &str,
        items: Vec<syn::Item>,
        modules: Vec<HirTreeModule>,
    ) -> HirTreeModule {
        HirTreeModule {
            meta: HirTreeModuleMeta {
                parent_vis: vec![],
                vis: HirVisibility::Public,
                namespace: Namespace::new_raw(namespace.to_owned()),
            },
            items,
            modules,
        }
    }

    /// Flattens root and nested module items while preserving their metadata.
    #[test]
    fn flattens_nested_modules_with_module_metadata() -> anyhow::Result<()> {
        let child = module(
            "crate::api",
            vec![parse_quote!(
                pub struct Child;
            )],
            vec![],
        );
        let root = module(
            "crate",
            vec![parse_quote!(
                pub struct Root;
            )],
            vec![child],
        );
        let pack = HirTreePack {
            crates: vec![HirTreeCrate {
                name: CrateName::self_crate(),
                root_module: root,
            }],
        };

        let flattened = parse(pack)?;

        assert_eq!(flattened.items.len(), 2);
        assert_eq!(flattened.items[0].meta.namespace.to_string(), "crate");
        assert_eq!(flattened.items[1].meta.namespace.to_string(), "crate::api");
        assert_eq!(
            flattened.items[1].meta.sources,
            vec![HirGenerationSource::Normal]
        );
        Ok(())
    }
}
