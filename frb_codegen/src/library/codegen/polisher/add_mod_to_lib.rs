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
                format!(
                    "rust_output_path={rust_output_path:?} is unrelated to path_src_folder={path_src_folder:?}",
                )
            })?;

    let mod_name = rust_output_path_relative_to_src_folder
        .file_stem()
        .context("No file_stem")?
        .to_str()
        .context("Not a UTF-8 path")?
        .to_string()
        .replace('/', "::");

    let path_lib_rs = path_src_folder.join("lib.rs");
    let raw_content_lib_rs = fs::read_to_string(&path_lib_rs)?;
    let lib_rs_content_normalized = raw_content_lib_rs.replace("\r\n", "\n");

    let final_content = process_lib_rs_content(&lib_rs_content_normalized, &mod_name);

    // --- Write back only if content actually changed from the original normalized state ---
    if final_content != lib_rs_content_normalized {
        fs::write(&path_lib_rs, final_content.as_bytes()).unwrap(); // Write as bytes to preserve LF newlines
    } else {
        info!("No changes needed to lib.rs. Skipping write.");
    }

    Ok(())
}

fn process_lib_rs_content(initial_content: &str, mod_name: &str) -> String {
    let mut current_content = initial_content.replace("\r\n", "\n"); // Normalize newlines to LF for consistent string operations

    let code_inject_block_marker_start = "// AUTO INJECTED BY flutter_rust_bridge. The following lines may not be accurate; change them according to your needs.";
    let code_inject_block_marker_end = "// END of AUTO INJECTED code";

    let code_to_inject = format!(
        "mod {mod_name};\n\
        // this export is needed for logging\n\
        pub use crate::{mod_name}::StreamSink as __FrbStreamSinkForLogging;"
    );

    let code_to_inject_full_block = format!(
        "{code_inject_block_marker_start}\n{code_to_inject}\n{code_inject_block_marker_end}\n"
    );

    // no injection needed, if code block is already present
    if !current_content.contains(&code_to_inject) {
        info!("Injecting needed dependencies into lib.rs.");

        // TODO remove after a transition period
        // code injected until FRB v.2.10
        let old_frb_injected_code = format!("mod {mod_name}; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */");

        let mut start_index = current_content.find(&old_frb_injected_code);
        let end_index = if start_index.is_some() {
            current_content.rfind(&old_frb_injected_code)
        } else {
            start_index = current_content.find(code_inject_block_marker_start);
            current_content.rfind(code_inject_block_marker_end)
        };

        current_content = match start_index {
            Some(start) => {
                // replace the old injected code with the new one
                current_content[..start - 1].to_string()
                    + &code_to_inject_full_block
                    + &current_content[end_index
                        .expect("lib.rs is wrong! Found inject code block start but not end!")..]
            }
            None =>
            // no old code to replace, just inject the new code
            {
                format!("{code_to_inject_full_block}\n{current_content}")
            }
        };
    } else {
        info!("New code block is already present and correct after cleanup. No further injection.");
    }
    current_content
}

#[cfg(test)]
mod tests {
    use super::*; // Import from the same module

    // Helper to get the expected new injected block for tests
    fn get_expected_new_block(mod_name: &str) -> String {
        format!(
            "// AUTO INJECTED BY flutter_rust_bridge. The following lines may not be accurate; change them according to your needs.\n\
             mod {mod_name};\n\
             // this export is needed for logging\n\
             pub use crate::{mod_name}::StreamSink as __FrbStreamSinkForLogging;\n\
             // END of AUTO INJECTED code\n",
        )
    }

    // --- Test Cases ---

    #[test]
    fn test_inject_into_empty_lib_rs() {
        let initial_content = "";
        let mod_name = "frb_generated";
        let result_content = process_lib_rs_content(initial_content, mod_name);
        assert_eq!(result_content, get_expected_new_block(mod_name));
    }

    #[test]
    fn test_inject_into_existing_lib_rs_without_newline_at_end() {
        let initial_content = "pub mod api;"; // No trailing newline
        let mod_name = "frb_generated";
        let result_content = process_lib_rs_content(initial_content, mod_name);
        let expected_code = format!("{}{}\n", get_expected_new_block(mod_name), initial_content); // New block + original + a final newline from filter
        assert_eq!(result_content, expected_code);
    }

    #[test]
    fn test_inject_into_existing_lib_rs_with_newline_at_end() {
        let initial_content = "pub mod api;\n"; // With trailing newline
        let mod_name = "frb_generated";
        let result_content = process_lib_rs_content(initial_content, mod_name);
        let expected_code = format!("{}{}", get_expected_new_block(mod_name), initial_content); // New block + original
        assert_eq!(result_content, expected_code);
    }

    #[test]
    fn test_no_change_if_already_up_to_date_new_format() {
        let mod_name = "frb_generated";
        let initial_content = format!(
            "{}\npub mod api;\n",                        // Existing new format
            get_expected_new_block(mod_name).trim_end() // Trim to match the prepend logic if it was appended
        );
        let result_content = process_lib_rs_content(&initial_content, mod_name);
        assert_eq!(result_content, initial_content); // Should remain unchanged
    }

    #[test]
    fn test_replace_old_format_with_new_format() {
        let mod_name = "frb_generated";
        let old_injected_line = format!(
            "mod {}; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */",
            mod_name
        );
        let initial_content = format!(
            "pub mod api;\n{}\nother code;", // Old format embedded
            old_injected_line
        );
        let result_content = process_lib_rs_content(&initial_content, mod_name);

        let expected_code = format!(
            "{}\npub mod api;\nother code;\n", // Old format removed, new prepended, remaining code at end
            get_expected_new_block(mod_name)
        );
        assert_eq!(result_content, expected_code);
    }

    #[test]
    fn test_replace_only_old_format_file() {
        let mod_name = "frb_generated";
        let old_injected_line = format!(
            "mod {}; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */",
            mod_name
        );
        let initial_content = format!("{}\n", old_injected_line); // File contains only the old format
        let result_content = process_lib_rs_content(&initial_content, mod_name);

        // Should be replaced by only the new block
        assert_eq!(result_content, get_expected_new_block(mod_name));
    }

    #[test]
    fn test_idempotent_after_replacement() {
        let mod_name = "frb_generated";
        let old_injected_line = format!(
            "mod {}; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */",
            mod_name
        );
        let initial_content = format!("pub mod api;\n{}\n", old_injected_line);

        let content_after_first_run = process_lib_rs_content(&initial_content, mod_name); // First run: replace old with new
        let content_after_second_run = process_lib_rs_content(&content_after_first_run, mod_name); // Second run: should not change

        assert_eq!(content_after_first_run, content_after_second_run);
        assert_eq!(
            content_after_second_run,
            format!("{}\npub mod api;\n", get_expected_new_block(mod_name))
        );
    }

    #[test]
    fn test_multiple_old_formats() {
        let mod_name = "frb_generated";
        let old_injected_line = format!(
            "mod {}; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */",
            mod_name
        );
        let initial_content = format!(
            "{}\npub mod api;\n{}\n", // Multiple old formats
            old_injected_line, old_injected_line
        );
        let result_content = process_lib_rs_content(&initial_content, mod_name);
        let expected_code = format!("{}\npub mod api;\n", get_expected_new_block(mod_name));
        assert_eq!(result_content, expected_code);
    }

    #[test]
    fn test_new_format_old_content() {
        let mod_name = "frb_generated";
        let initial_content = 
        format!(
            "// AUTO INJECTED BY flutter_rust_bridge. The following lines may not be accurate; change them according to your needs.\n\
             mod {mod_name};\n\
             // this export is needed for logging\n\
             pub use crate::{mod_name}::StreamSink as __FrbStreamSinkForLogging;\n\
             // END of AUTO INJECTED code\n\
             pub mod api;\n"
        );
        let result_content = process_lib_rs_content(&initial_content, mod_name);
        let expected_code = format!("{}\npub mod api;\n", get_expected_new_block(mod_name));
        assert_eq!(result_content, expected_code);
    }

    #[test]
    fn test_mixed_formats_and_other_content() {
        let mod_name = "frb_generated";
        let old_injected_line = format!(
            "mod {mod_name}; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */"
        );
        let existing_new_block = get_expected_new_block(mod_name);

        let initial_content = format!(
            "// Some header comments\n\n\
             {}\n\n\
             pub mod feature_a;\n\n\
             {}\n\n\
             pub mod feature_b;\n\n\
             {}\n\n\
             // Some footer comments\n",
            existing_new_block, // Existing new block
            old_injected_line,  // Old format
            old_injected_line   // Another old format
        );
        let result_content = process_lib_rs_content(&initial_content, mod_name);

        let expected_code = format!(
            "// Some header comments\n\n\
             {}\n\n\
             pub mod feature_a;\n\n\
             pub mod feature_b;\n\n\
             // Some footer comments\n",
            existing_new_block
        );
        assert_eq!(result_content, expected_code);
    }
}
