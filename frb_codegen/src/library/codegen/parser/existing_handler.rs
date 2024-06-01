use crate::codegen::ir::hir::hierarchical::module::HirModule;
use crate::codegen::ir::mir::namespace::{Namespace, NamespacedName};
use crate::codegen::parser::internal_config::ParserInternalConfig;
use crate::library::misc::consts::HANDLER_NAME;
use anyhow::ensure;
use itertools::Itertools;

pub(super) fn parse_existing_handlers(
    config: &ParserInternalConfig,
    modules: &[&HirModule],
) -> anyhow::Result<Vec<NamespacedName>> {
    let existing_handlers = (modules.iter())
        .filter(|module| module.raw.iter().any(|code| parse_has_executor(code)))
        .map(|module| {
            NamespacedName::new(module.meta.namespace.to_owned(), HANDLER_NAME.to_owned())
        })
        .collect_vec();
    ensure!(
        existing_handlers.len() <= 1,
        // frb-coverage:ignore-start
        // This will stop the whole generator and tell the users, so we do not care about testing it
        "Should have at most one custom handler"
    );
    // frb-coverage:ignore-end
    Ok(existing_handlers)
}

fn parse_has_executor(code: &str) -> bool {
    code.contains(&format!("static {HANDLER_NAME}"))
}
