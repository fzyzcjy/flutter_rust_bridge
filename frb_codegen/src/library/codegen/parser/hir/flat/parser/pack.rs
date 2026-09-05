use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::naive_flat::pack::HirNaiveFlatPack;
use crate::codegen::parser::hir::flat::parser::syn_item::parse_syn_item;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;

pub(crate) fn parse_pack(
    config: &ParserHirInternalConfig,
    hir_naive_flat: HirNaiveFlatPack,
) -> anyhow::Result<HirFlatPack> {
    let mut pack = HirFlatPack {
        existing_handler: super::existing_handler::parse_existing_handler(
            &hir_naive_flat.items,
            config,
        )?,
        ..HirFlatPack::default()
    };

    for item in hir_naive_flat.items {
        parse_syn_item(item.item, &item.meta, &mut pack, config.parse_const)?;
    }

    Ok(pack)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::ir::hir::misc::generation_source::HirGenerationSource;
    use crate::codegen::ir::hir::naive_flat::item::{HirNaiveFlatItem, HirNaiveFlatItemMeta};
    use crate::codegen::parser::mir::internal_config::RustInputNamespacePack;
    use crate::utils::namespace::Namespace;
    use std::path::PathBuf;
    use syn::parse_quote;

    /// Parses all naive items with the configured constant policy.
    #[test]
    fn parses_naive_items_into_flat_pack() {
        let config = ParserHirInternalConfig {
            rust_input_namespace_pack: RustInputNamespacePack {
                rust_input_namespace_prefixes: vec![Namespace::new_raw("crate::api".to_owned())],
                rust_output_path_namespace: Namespace::new_raw("crate::generated".to_owned()),
            },
            rust_crate_dir: PathBuf::new(),
            third_party_crate_names: vec![],
            rust_features: None,
            parse_const: true,
        };
        let meta = HirNaiveFlatItemMeta {
            namespace: Namespace::new_raw("crate::api".to_owned()),
            sources: vec![HirGenerationSource::Normal],
            is_module_public: true,
        };
        let pack = parse_pack(
            &config,
            HirNaiveFlatPack {
                items: vec![
                    HirNaiveFlatItem {
                        meta: meta.clone(),
                        item: parse_quote!(
                            pub fn run() {}
                        ),
                    },
                    HirNaiveFlatItem {
                        meta,
                        item: parse_quote!(
                            pub const ID: u8 = 1;
                        ),
                    },
                ],
            },
        )
        .unwrap();

        assert_eq!(pack.functions.len(), 1);
        assert_eq!(pack.constants.len(), 1);
    }
}
