use crate::codegen::ir::hir::raw::pack::HirRawPack;
use crate::codegen::ir::hir::tree::pack::HirTreePack;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::codegen::parser::hir::tree::parser::crates::parse_crate;

pub(crate) fn parse_pack(
    config: &ParserHirInternalConfig,
    hir_raw: HirRawPack,
) -> anyhow::Result<HirTreePack> {
    let crates = (hir_raw.crates.into_iter())
        .map(|c| parse_crate(config, c.syn_file, &c.name))
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .collect();
    Ok(HirTreePack { crates })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::ir::hir::raw::crates::HirRawCrate;
    use crate::codegen::parser::mir::internal_config::RustInputNamespacePack;
    use crate::utils::crate_name::CrateName;
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

    /// Converts every raw crate into a tree crate without changing their order.
    #[test]
    fn parses_each_raw_crate_in_order() -> anyhow::Result<()> {
        let parsed = parse_pack(
            &config(),
            HirRawPack {
                crates: vec![
                    HirRawCrate {
                        name: CrateName::self_crate(),
                        syn_file: parse_quote!(
                            pub struct Root;
                        ),
                    },
                    HirRawCrate {
                        name: CrateName::new("dependency".to_owned()),
                        syn_file: parse_quote!(
                            pub struct Dependency;
                        ),
                    },
                ],
            },
        )?;

        assert_eq!(parsed.crates.len(), 2);
        assert_eq!(parsed.crates[0].name.raw(), "crate");
        assert_eq!(parsed.crates[1].name.raw(), "dependency");
        assert_eq!(parsed.crates[1].root_module.items.len(), 1);
        Ok(())
    }
}
