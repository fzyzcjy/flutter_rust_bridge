use crate::utils::path_utils::path_to_string;
use anyhow::Context;
use std::path::Path;

pub(crate) fn compute_mod_from_rust_crate_path(
    code_path: &Path,
    rust_crate_dir: &Path,
) -> anyhow::Result<String> {
    compute_mod_from_path(code_path, &rust_crate_dir.join("src"))
}

fn compute_mod_from_path(code_path: &Path, base_dir: &Path) -> anyhow::Result<String> {
    (|| -> anyhow::Result<String> {
        let p = code_path.strip_prefix(base_dir)?.with_extension("");
        Ok(path_to_string(&p)?.replace('/', "::"))
    })()
    .with_context(|| {
        format!("When compute_mod_from_rust_path(code_path={code_path:?}, base_dir={base_dir:?})",)
    })
}
