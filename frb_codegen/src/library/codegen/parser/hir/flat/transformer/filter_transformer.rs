use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::misc::SELF_CRATE_THIRD_PARTY_NAMESPACE;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::utils::namespace::Namespace;
use itertools::Itertools;
use syn::Visibility;

pub(crate) fn transform(
    mut pack: HirFlatPack,
    config: &ParserHirInternalConfig,
) -> anyhow::Result<HirFlatPack> {
    filter_function(&mut pack, config);
    filter_constant(&mut pack, config);
    Ok(pack)
}

fn filter_function(pack: &mut HirFlatPack, config: &ParserHirInternalConfig) {
    pack.functions = (pack.functions.drain(..))
        .filter(|x| should_keep(&x.namespace, x.is_public().unwrap_or(true), config))
        .collect_vec();
}

fn filter_constant(pack: &mut HirFlatPack, config: &ParserHirInternalConfig) {
    pack.constants = (pack.constants.drain(..))
        .filter(|x| {
            should_keep(
                &x.namespace,
                matches!(x.item_const.vis, Visibility::Public(_)),
                config,
            )
        })
        .collect_vec();
}

fn should_keep(namespace: &Namespace, is_public: bool, config: &ParserHirInternalConfig) -> bool {
    is_interest_module(namespace, config) && (namespace.crate_name().is_self_crate() || is_public)
}

fn is_interest_module(namespace: &Namespace, config: &ParserHirInternalConfig) -> bool {
    (config.rust_input_namespace_pack).is_interest(namespace)
        || SELF_CRATE_THIRD_PARTY_NAMESPACE.is_prefix_of(namespace)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::ir::hir::flat::constant::HirFlatConstant;
    use crate::codegen::ir::hir::flat::function::{HirFlatFunction, HirFlatFunctionOwner};
    use crate::codegen::ir::hir::misc::item_fn::GeneralizedItemFn;
    use crate::codegen::parser::mir::internal_config::RustInputNamespacePack;
    use std::path::PathBuf;
    use syn::parse_quote;

    fn config() -> ParserHirInternalConfig {
        ParserHirInternalConfig {
            rust_input_namespace_pack: RustInputNamespacePack {
                rust_input_namespace_prefixes: vec![
                    Namespace::new_raw("crate::api".to_owned()),
                    Namespace::new_raw("dependency::api".to_owned()),
                ],
                rust_output_path_namespace: Namespace::new_raw("crate::generated".to_owned()),
            },
            rust_crate_dir: PathBuf::new(),
            third_party_crate_names: vec![],
            rust_features: None,
            parse_const: true,
        }
    }

    fn function(namespace: &str, item_fn: syn::ItemFn) -> HirFlatFunction {
        HirFlatFunction {
            namespace: Namespace::new_raw(namespace.to_owned()),
            owner: HirFlatFunctionOwner::Function,
            sources: vec![],
            item_fn: GeneralizedItemFn::ItemFn(item_fn),
        }
    }

    /// Removes private external and uninteresting functions.
    #[test]
    fn filters_functions_by_namespace_and_visibility() {
        let pack = HirFlatPack {
            functions: vec![
                function(
                    "crate::api",
                    parse_quote!(
                        fn self_private() {}
                    ),
                ),
                function(
                    "dependency::api",
                    parse_quote!(
                        pub fn external_public() {}
                    ),
                ),
                function(
                    "dependency::api",
                    parse_quote!(
                        fn external_private() {}
                    ),
                ),
                function(
                    "crate::other",
                    parse_quote!(
                        pub fn uninteresting() {}
                    ),
                ),
            ],
            ..Default::default()
        };

        let transformed = transform(pack, &config()).unwrap();

        assert_eq!(
            transformed
                .functions
                .iter()
                .map(|x| x.item_fn.name())
                .collect_vec(),
            vec!["self_private", "external_public"]
        );
    }

    /// Removes constants with non-public visibility outside the self crate.
    #[test]
    fn filters_constants_by_namespace_and_visibility() {
        let pack = HirFlatPack {
            constants: vec![
                HirFlatConstant {
                    namespace: Namespace::new_raw("crate::api".to_owned()),
                    item_const: parse_quote!(
                        const SELF_PRIVATE: u8 = 1;
                    ),
                },
                HirFlatConstant {
                    namespace: Namespace::new_raw("dependency::api".to_owned()),
                    item_const: parse_quote!(
                        pub const EXTERNAL_PUBLIC: u8 = 1;
                    ),
                },
                HirFlatConstant {
                    namespace: Namespace::new_raw("dependency::api".to_owned()),
                    item_const: parse_quote!(
                        const EXTERNAL_PRIVATE: u8 = 1;
                    ),
                },
            ],
            ..Default::default()
        };

        let transformed = transform(pack, &config()).unwrap();

        assert_eq!(
            transformed
                .constants
                .iter()
                .map(|x| x.item_const.ident.to_string())
                .collect_vec(),
            vec!["SELF_PRIVATE", "EXTERNAL_PUBLIC"]
        );
    }
}
