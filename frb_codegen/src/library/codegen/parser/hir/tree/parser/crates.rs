use crate::codegen::ir::hir::misc::visibility::HirVisibility;
use crate::codegen::ir::hir::tree::crates::HirTreeCrate;
use crate::codegen::ir::hir::tree::module::HirTreeModuleMeta;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::codegen::parser::hir::tree::parser::module::parse_module;
use crate::utils::crate_name::CrateName;

pub(crate) fn parse_crate(
    config: &ParserHirInternalConfig,
    file: syn::File,
    crate_name: &CrateName,
) -> anyhow::Result<HirTreeCrate> {
    let info = HirTreeModuleMeta {
        parent_vis: vec![],
        vis: HirVisibility::Public,
        namespace: crate_name.namespace(),
    };
    let root_module = parse_module(file.items, info, config)?;
    Ok(HirTreeCrate {
        name: crate_name.to_owned(),
        root_module,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::parser::mir::internal_config::RustInputNamespacePack;
    use crate::utils::namespace::Namespace;
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

    /// Creates a public root module in the namespace derived from the crate name.
    #[test]
    fn parses_crate_root_with_its_crate_namespace() -> anyhow::Result<()> {
        let parsed = parse_crate(
            &config(),
            parse_quote!(
                pub struct Api;
            ),
            &CrateName::new("external-crate".to_owned()),
        )?;

        assert_eq!(parsed.name.raw(), "external-crate");
        assert_eq!(
            parsed.root_module.meta.namespace.to_string(),
            "external_crate"
        );
        assert!(parsed.root_module.meta.is_public());
        assert_eq!(parsed.root_module.items.len(), 1);
        Ok(())
    }
}
