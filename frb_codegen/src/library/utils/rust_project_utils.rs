use crate::utils::path_utils::path_to_string;
use anyhow::Context;
use std::path::Path;

pub(crate) fn compute_mod_from_rust_path(
    code_path: &Path,
    crate_path: &Path,
) -> anyhow::Result<String> {
    (|| -> anyhow::Result<String> {
        let p = code_path
            .strip_prefix(crate_path.join("src"))?
            .with_extension("");
        Ok(path_to_string(&p)?.replace('/', "::"))
    })()
    .with_context(|| {
        format!(
            "When compute_mod_from_rust_path(code_path={code_path:?}, crate_path={crate_path:?})",
        )
    })
}
