use crate::codegen::parser::mir::internal_config::RustInputNamespacePack;
use crate::library::misc::consts::HANDLER_NAME;
use crate::utils::namespace::NamespacedName;
use anyhow::ensure;
use itertools::Itertools;
use crate::codegen::ir::hir::tree::module::HirTreeModule;

pub(super) fn parse_existing_handlers(
    modules: &[&HirTreeModule],
    rust_input_namespace_pack: &RustInputNamespacePack,
) -> anyhow::Result<Vec<NamespacedName>> {
    let existing_handlers = (modules.iter())
        .filter(|module| rust_input_namespace_pack.is_interest(&module.meta.namespace))
        .filter(|module| module.raw.iter().any(|code| parse_has_executor(code)))
        .map(|module| {
            NamespacedName::new(module.meta.namespace.to_owned(), HANDLER_NAME.to_owned())
        })
        .collect_vec();
    ensure!(
        existing_handlers.len() <= 1,
        // frb-coverage:ignore-start
        // This will stop the whole generator and tell the users, so we do not care about testing it
        "Should have at most one custom handler (currently: {})",
        existing_handlers.iter().map(|x| x.rust_style()).join(", ")
    );
    // frb-coverage:ignore-end
    Ok(existing_handlers)
}

fn parse_has_executor(code: &str) -> bool {
    code.contains(&format!("static {HANDLER_NAME}"))
}
