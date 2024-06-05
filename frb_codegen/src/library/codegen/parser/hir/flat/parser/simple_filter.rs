use crate::codegen::parser::hir::flat::parser::flattener::SynItemWithMeta;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;

pub(crate) fn simple_filter(
    items: Vec<SynItemWithMeta>,
    config: &ParserHirInternalConfig,
) -> Vec<SynItemWithMeta> {
    (items.into_iter())
        .filter(|item| (config.rust_input_namespace_pack).is_interest(item.meta.namespace))
        .collect()
}
