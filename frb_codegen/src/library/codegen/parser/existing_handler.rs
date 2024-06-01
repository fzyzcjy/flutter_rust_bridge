use crate::codegen::ir::namespace::{Namespace, NamespacedName};
use crate::codegen::parser::internal_config::ParserInternalConfig;
use crate::library::misc::consts::HANDLER_NAME;
use anyhow::ensure;
use itertools::Itertools;

pub(super) fn parse_existing_handlers(
    config: &ParserInternalConfig,
    file_data_arr: &[FileData],
) -> anyhow::Result<Vec<NamespacedName>> {
    let existing_handlers = (file_data_arr.iter())
        .filter(|file| parse_has_executor(&file.content))
        .map(|file| {
            NamespacedName::new(
                Namespace::new_from_rust_crate_path(&file.path, &config.rust_crate_dir).unwrap(),
                HANDLER_NAME.to_owned(),
            )
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

fn parse_has_executor(source_rust_content: &str) -> bool {
    source_rust_content.contains(&format!("static {HANDLER_NAME}"))
}
