use crate::utils::path_utils::path_to_string;
use anyhow::Context;
use std::path::Path;

pub(crate) fn compute_mod_from_path(code_path: &Path, root_path: &Path) -> anyhow::Result<String> {
    (|| -> anyhow::Result<String> {
        let join_dir = match lang {
            ProgrammingLang::Dart => "lib",
            ProgrammingLang::Rust => "src",
            ProgrammingLang::C => unimplemented!(),
        };
        let p = code_path
            .strip_prefix(root_path.join(join_dir))?
            .with_extension("");
        Ok(path_to_string(&p)?.replace('/', "::"))
    })()
    .with_context(|| {
        format!(
            "When compute_mod_from_rust_path(code_path={code_path:?}, root_path={root_path:?})",
        )
    })
}
