use crate::codegen::ir::hir::misc::visibility::HirVisibility;
use crate::codegen::ir::hir::tree::module::{HirTreeModule, HirTreeModuleMeta};
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
use crate::utils::namespace::Namespace;
use syn::ItemMod;

pub(super) fn parse_module(
    items: Vec<syn::Item>,
    meta: HirTreeModuleMeta,
    config: &ParserHirInternalConfig,
) -> anyhow::Result<HirTreeModule> {
    let mut output_items = vec![];
    let mut output_modules = vec![];

    for item in items.into_iter() {
        match item {
            syn::Item::Mod(item_mod) => output_modules.extend(parse_syn_item_mod(
                item_mod,
                config,
                &meta.namespace,
                &meta.parent_vis,
            )?),
            _ => output_items.push(item),
        }
    }

    Ok(HirTreeModule {
        meta,
        items: output_items,
        modules: output_modules,
    })
}

fn parse_syn_item_mod(
    item_mod: ItemMod,
    config: &ParserHirInternalConfig,
    namespace: &Namespace,
    parent_vis: &[HirVisibility],
) -> anyhow::Result<Option<HirTreeModule>> {
    if let Some((_, items)) = item_mod.content {
        if !FrbAttributes::parse(&item_mod.attrs)?.ignore() {
            let info = HirTreeModuleMeta {
                parent_vis: parent_vis.to_owned(),
                vis: (&item_mod.vis).into(),
                namespace: namespace.join(&item_mod.ident.to_string()),
            };
            return Ok(Some(parse_module(items, info, config)?));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::parser::mir::internal_config::RustInputNamespacePack;
    use std::path::PathBuf;
    use syn::parse_quote;

    fn config() -> ParserHirInternalConfig {
        ParserHirInternalConfig {
            rust_input_namespace_pack: RustInputNamespacePack {
                rust_input_namespace_prefixes: vec![],
                rust_output_path_namespace: Namespace::new_raw("crate::generated".to_owned()),
            },
            rust_crate_dir: PathBuf::new(),
            third_party_crate_names: vec![],
            rust_features: None,
            parse_const: false,
        }
    }

    fn root_meta() -> HirTreeModuleMeta {
        HirTreeModuleMeta {
            parent_vis: vec![],
            vis: HirVisibility::Public,
            namespace: Namespace::new_raw("crate".to_owned()),
        }
    }

    /// Keeps non-module items and recursively records inline non-ignored modules.
    #[test]
    fn parses_inline_modules_and_skips_ignored_or_external_modules() -> anyhow::Result<()> {
        let parsed = parse_module(
            vec![
                parse_quote!(
                    pub struct Root;
                ),
                parse_quote!(
                    pub mod api {
                        pub struct Api;
                    }
                ),
                parse_quote!(
                    #[frb(ignore)]
                    pub mod skipped {
                        pub struct Skipped;
                    }
                ),
                parse_quote!(
                    pub mod external;
                ),
            ],
            root_meta(),
            &config(),
        )?;

        assert_eq!(parsed.items.len(), 1);
        assert_eq!(parsed.modules.len(), 1);
        assert_eq!(parsed.modules[0].meta.namespace.to_string(), "crate::api");
        assert_eq!(parsed.modules[0].items.len(), 1);
        assert!(parsed.modules[0].meta.is_public());
        Ok(())
    }
}
