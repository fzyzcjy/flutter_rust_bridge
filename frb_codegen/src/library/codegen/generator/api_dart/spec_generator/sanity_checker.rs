use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use anyhow::bail;
use itertools::Itertools;
use log::warn;

pub(crate) fn sanity_check_class_name_duplicates(
    classes: &[ApiDartGeneratedClass],
) -> anyhow::Result<()> {
    let duplicate_class_names = classes
        .iter()
        .map(|c| c.class_name.clone())
        .duplicates()
        .collect_vec();

    // This will stop the whole generator and tell the users, so we do not care about testing it
    // frb-coverage:ignore-start
    if !duplicate_class_names.is_empty() {
        const SKIP_ENV_VAR: &str = "FRB_DEBUG_SKIP_SANITY_CHECK_CLASS_NAME_DUPLICATES";
        let message = format!(
            "Will generate duplicated class names ({:?}). This is often because the type is auto inferred as both opaque and non-opaque. Try to add `#[frb(opaque)]` or `#[frb(non_opaque)]` to the struct, or change code that uses it.",
            duplicate_class_names,
        );

        if std::env::var(SKIP_ENV_VAR).is_ok() {
            warn!("{}", message);
        } else {
            bail!(
                "{}Another way to debug is to temporarily set environment variable `{}=1` and check the generated code.",
                message,
                SKIP_ENV_VAR
            );
        }
    }
    // frb-coverage:ignore-end

    Ok(())
}
