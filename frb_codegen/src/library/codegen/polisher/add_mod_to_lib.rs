use anyhow::*;
use log::{info, warn};
use pathdiff::diff_paths;
use std::fs;
use std::path::Path;

// the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
// frb-coverage:ignore-start
pub(super) fn try_add_mod_to_lib(rust_crate_dir: &Path, rust_output_path: &Path) {
    // frb-coverage:ignore-end
    if let Err(e) = auto_add_mod_to_lib_core(rust_crate_dir, rust_output_path) {
        // We do not care about the warning
        // frb-coverage:ignore-start
        warn!(
            "add_mod_to_lib fail, the generated code may or may not have problems. \
            Please ensure you have add code like `mod the_generated_bridge_code;` to your `lib.rs`. \
            Details: {e}"
        );
        // frb-coverage:ignore-end
    }
}

fn auto_add_mod_to_lib_core(rust_crate_dir: &Path, rust_output_path: &Path) -> Result<()> {
    let path_src_folder = rust_crate_dir.join("src");
    let rust_output_path_relative_to_src_folder =
        diff_paths(rust_output_path, path_src_folder.clone()).with_context(|| {
            // This will stop the whole generator and tell the users, so we do not care about testing it
            // frb-coverage:ignore-start
            format!(
                "rust_output_path={rust_output_path:?} is unrelated to path_src_folder={path_src_folder:?}",
            )
            // frb-coverage:ignore-end
        })?;

    let mod_name = rust_output_path_relative_to_src_folder
        .file_stem()
        .context("No file_stem")?
        .to_str()
        .context("Not a UTF-8 path")?
        .to_string()
        .replace('/', "::");
    let expect_code = format!(
        r#"mod {mod_name};
// this export is needed for logging
pub use crate::{mod_name}::StreamSink as __FrbStreamSinkForLogging;"#
    );

    let path_lib_rs = path_src_folder.join("lib.rs");

    let raw_content_lib_rs = fs::read_to_string(path_lib_rs.clone())?;
    if !raw_content_lib_rs.contains(&expect_code) {
        info!("Inject `{}` into {:?}", &expect_code, &path_lib_rs);
        let modified_content_lib_rs = format!(
            "// AUTO INJECTED BY flutter_rust_bridge. The following lines may not be accurate; change them according to your needs.\n{}\n{}\n{}",
            expect_code,
            "// END of AUTO INJECTED code",
            {raw_content_lib_rs}
        );
        fs::write(&path_lib_rs, modified_content_lib_rs).unwrap();
    }

    Ok(())
}
