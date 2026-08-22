use crate::codegen::ir::hir::flat::function::{HirFlatFunction, HirFlatFunctionOwner};
use crate::codegen::ir::hir::misc::item_fn::GeneralizedItemFn;
use crate::codegen::ir::hir::naive_flat::item::HirNaiveFlatItemMeta;
use syn::ItemFn;

pub(crate) fn parse_syn_item_fn(item_fn: ItemFn, meta: &HirNaiveFlatItemMeta) -> HirFlatFunction {
    HirFlatFunction {
        namespace: meta.namespace.clone(),
        owner: HirFlatFunctionOwner::Function,
        item_fn: GeneralizedItemFn::ItemFn(item_fn),
        sources: meta.sources.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::ir::hir::misc::generation_source::HirGenerationSource;
    use crate::utils::namespace::Namespace;
    use syn::parse_quote;

    /// Creates a free-function owner while preserving metadata.
    #[test]
    fn parses_free_function_with_metadata() {
        let meta = HirNaiveFlatItemMeta {
            namespace: Namespace::new_raw("crate::api".to_owned()),
            sources: vec![HirGenerationSource::Normal],
            is_module_public: true,
        };
        let parsed = parse_syn_item_fn(
            parse_quote!(
                pub fn hello() {}
            ),
            &meta,
        );

        assert!(matches!(parsed.owner, HirFlatFunctionOwner::Function));
        assert_eq!(parsed.namespace, meta.namespace);
        assert_eq!(parsed.sources, meta.sources);
        assert_eq!(parsed.item_fn.name(), "hello");
    }
}
