use anyhow::*;
use log::{debug, info, warn};
use pathdiff::diff_paths;
use std::fs;
use std::path::{Component, Path};

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

    let path_lib_rs = path_src_folder.join("lib.rs");
    let raw_content_lib_rs = fs::read_to_string(&path_lib_rs)?;
    let lib_rs_content_normalized = raw_content_lib_rs.replace("\r\n", "\n");

    let final_content = process_lib_rs_content(
        &lib_rs_content_normalized,
        &rust_output_path_relative_to_src_folder,
    )?;

    // --- Write back only if content actually changed from the original normalized state, ignoring white spaces ---
    if final_content.replace("/n", "") != lib_rs_content_normalized.replace("/n", "") {
        fs::write(&path_lib_rs, final_content.as_bytes()).unwrap(); // Write as bytes to preserve LF newlines
    } else {
        info!("No changes needed to lib.rs. Skipping write.");
    }

    Ok(())
}

fn process_lib_rs_content(initial_content: &str, relative_path: &Path) -> Result<String> {
    const CODE_INJECT_BLOCK_MARKER_START: &str = "// AUTO INJECTED BY flutter_rust_bridge.";
    const CODE_INJECT_BLOCK_MARKER_END: &str = "// END of AUTO INJECTED code";

    let relative_path = &relative_path.with_extension("");

    let (mod_name, path) = if relative_path
        .parent()
        // test is path is just ""
        .is_some_and(|parents| parents.parent().is_some())
    {
        // Path has a parent, meaning it's not just a filename
        // mod_name: first component of the original relative_path (e.g., "bridge" from "bridge/gen/frb_generated.rs")
        let mod_name_first_component = relative_path
            .components()
            .next() // Get the first component of the *original* relative_path
            .and_then(|component| match component {
                Component::Normal(s) => s.to_str(),
                _ => {
                    unreachable!("The path should be relative!")
                }
            })
            .context("First component of path is not valid UTF-8 or could not be extracted")?
            .to_string();

        // path: the full parent path, without the filename, with "::" separators
        let path_string_with_colons = relative_path
            .to_str()
            .context("the path is not valid UTF-8")?
            .replace(std::path::MAIN_SEPARATOR, "::");

        Ok((mod_name_first_component, Some(path_string_with_colons)))
    } else {
        // Path has no parent, meaning it's a filename only (e.g., "frb_generated.rs")
        // mod_name: the file_stem (filename without extension)
        Ok((
            relative_path
                .to_str()
                .context("File stem is not valid UTF-8")?
                .to_string(),
            None,
        ))
    }?;

    let code_to_inject = format!(
        "// The following lines may not be accurate; change them according to your needs.\n\
        mod {mod_name};{}",
        path.map_or_else(
            || "".to_string(),
            |path| {
                format!(
                    "\n\
            use crate::{path};"
                )
            }
        )
    );

    let code_to_inject_full_block = format!(
        "{CODE_INJECT_BLOCK_MARKER_START}\n{code_to_inject}\n{CODE_INJECT_BLOCK_MARKER_END}\n\n"
    );

    // TODO remove this lines after a migration period
    // remove FRB's per-v2.10.0 injectedd code
    let legacy_frb_injected_code = format!("mod {}; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */"
                                                    , relative_path.to_str().context("File stem is not valid UTF-8")?);
    let mut in_injected_block = false;
    let mut new_code_injected = false;
    let mut pre_code_injection_text = String::new();
    let mut output = String::new();
    let mut last_line_was_empty = true;

    debug!("injecting code to lib.rs");
    for line in initial_content.lines() {
        // trace!("line is: '{line}'");
        match line.trim() {
            "" => {
                // Only add empty line if the last line wasn't empty (collapse multiple empty lines)
                if !last_line_was_empty {
                    output.push('\n'); // normalises \r\n to \n
                    last_line_was_empty = true;
                }
            }
            // Keep config lines (e.g. `#![allow(clippy::new_without_default)]`) on top of the file
            trimmed_line if trimmed_line.starts_with("#!") => {
                pre_code_injection_text.push_str(trimmed_line);
                pre_code_injection_text.push('\n');
            }
            trimmed_line if trimmed_line == legacy_frb_injected_code => {
                debug!("legacy code, removing");
                last_line_was_empty = false;
            }
            trimmed_line if trimmed_line == CODE_INJECT_BLOCK_MARKER_START => {
                if !in_injected_block {
                    in_injected_block = true;
                }
            }
            trimmed_line if trimmed_line == CODE_INJECT_BLOCK_MARKER_END => {
                if in_injected_block {
                    if !new_code_injected {
                        // overwriter old injected code with new one
                        output.push_str(&_inject_code(
                            &pre_code_injection_text,
                            &code_to_inject_full_block,
                        ));
                        debug!("INJECT code");
                        new_code_injected = true;
                    }
                    in_injected_block = false;
                    last_line_was_empty = true;
                } else {
                    panic!("\nCould not generate code for lib.rs, as the content is wrong (missing injected code start marker '{CODE_INJECT_BLOCK_MARKER_START}'.\nThis is the content:\n'\n{initial_content}\n'");
                }
            }
            trimmed_line if !in_injected_block => {
                // take line outside of injected code and trim it
                output.push_str(&format!("{trimmed_line}\n"));
                last_line_was_empty = false;
            }
            _ if in_injected_block => {
                // drop the old injected line
            }
            _ => {
                unreachable!()
            }
        }
        // debug!("output is: '{output}'");
    }
    if in_injected_block {
        panic!("\nCould not generate code for lib.rs, as the content is wrong (missing injected code end marker '{CODE_INJECT_BLOCK_MARKER_END}'.\nThis is the content:\n'\n{initial_content}\n'");
    }

    if !new_code_injected {
        output = format!(
            "{}{output}",
            _inject_code(&pre_code_injection_text, &code_to_inject_full_block)
        );
    };
    // have only one neline in the very end
    output = format!("{}\n", output.trim_end_matches('\n'));
    Ok(output)
}

fn _inject_code(pre_code_injection_text: &str, code_to_inject: &str) -> String {
    format!(
        "{pre_code_injection_text}{}{code_to_inject}",
        if pre_code_injection_text.is_empty() {
            ""
        } else {
            "\n"
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to get the expected new injected block for tests
    fn get_expected_new_block_filename_only(mod_name: &Path) -> String {
        format!(
            "// AUTO INJECTED BY flutter_rust_bridge.\n\
             // The following lines may not be accurate; change them according to your needs.\n\
             mod {};\n\
             // END of AUTO INJECTED code\n",
            mod_name.with_extension("").to_str().unwrap()
        )
    }
    fn get_expected_new_block_with_path(mod_name: &Path, path: &Path) -> String {
        format!(
            "// AUTO INJECTED BY flutter_rust_bridge.\n\
             // The following lines may not be accurate; change them according to your needs.\n\
             mod {};\n\
             use crate::{};\n\
             // END of AUTO INJECTED code\n",
            mod_name.to_str().unwrap(),
            path.to_str()
                .unwrap()
                .replace(std::path::MAIN_SEPARATOR, "::")
        )
    }

    fn get_expected_full_file_filename_only(mod_name: &Path) -> String {
        format!(
            "{}\npub mod api;\n",
            get_expected_new_block_filename_only(mod_name)
        )
    }
    fn get_expected_full_file_with_path(mod_name: &Path, path: &Path) -> String {
        format!(
            "{}\npub mod api;\n",
            get_expected_new_block_with_path(mod_name, path)
        )
    }

    // --- Test Cases ---

    #[test]
    fn test_inject_into_empty_lib_rs() {
        let initial_content = "";
        let path = Path::new("frb_generated.rs");
        let result_content = process_lib_rs_content(initial_content, path).unwrap();
        assert_eq!(result_content, get_expected_new_block_filename_only(path));
    }

    #[test]
    fn test_inject_into_existing_lib_rs_without_newline_at_end() {
        let initial_content = "pub mod api;"; // No trailing newline
        let path = Path::new("frb_generated");
        let result_content = process_lib_rs_content(initial_content, path).unwrap();
        assert_eq!(result_content, get_expected_full_file_filename_only(path));
    }

    #[test]
    fn test_inject_into_existing_lib_rs_with_path() {
        let initial_content = "pub mod api;"; // No trailing newline
        let path = Path::new("bridge/gen/frb_generated.rs");
        let result_content = process_lib_rs_content(initial_content, path).unwrap();
        assert_eq!(
            result_content,
            get_expected_full_file_with_path(
                Path::new("bridge"),
                path.with_extension("").as_path()
            )
        );
    }
    #[test]
    fn test_inject_into_existing_lib_rs_with_one_path() {
        let initial_content = "pub mod api;"; // No trailing newline
        let path = Path::new("gen/frb_generated.rs");
        let result_content = process_lib_rs_content(initial_content, path).unwrap();
        assert_eq!(
            result_content,
            get_expected_full_file_with_path(Path::new("gen"), path.with_extension("").as_path())
        );
    }

    #[test]
    fn test_inject_into_existing_lib_rs_with_newline_at_end() {
        let initial_content = "pub mod api;\n"; // With trailing newline
        let path = Path::new("frb_generated");
        let result_content = process_lib_rs_content(initial_content, path).unwrap();
        assert_eq!(result_content, get_expected_full_file_filename_only(path));
    }
    #[test]
    fn test_inject_into_existing_lib_rs_with_multiple_newlines_at_end() {
        let initial_content = "pub mod api;\n\n\n"; // With trailing newline
        let path = Path::new("frb_generated");
        let result_content = process_lib_rs_content(initial_content, path).unwrap();
        assert_eq!(result_content, get_expected_full_file_filename_only(path));
    }
    #[test]
    fn test_inject_into_existing_lib_rs_with_multiple_newlines() {
        let initial_content = "\n\n\npub mod api;\n\n\n"; // With trailing newline
        let path = Path::new("frb_generated");
        let result_content = process_lib_rs_content(initial_content, path).unwrap();
        assert_eq!(result_content, get_expected_full_file_filename_only(path));
    }

    #[test]
    fn test_no_change_if_already_up_to_date_new_format() {
        let path = Path::new("frb_generated");
        let initial_content = get_expected_full_file_filename_only(path);
        let result_content = process_lib_rs_content(&initial_content, path).unwrap();
        assert_eq!(result_content, initial_content); // Should remain unchanged
    }

    #[test]
    fn test_leave_at_top() {
        let path = Path::new("frb_generated");
        let result_content =
            process_lib_rs_content(&get_expected_full_file_filename_only(path), path).unwrap();
        assert_eq!(result_content, get_expected_full_file_filename_only(path)); // Should remain unchanged
    }

    #[test]
    fn test_replace_old_format_with_new_format() {
        let path = Path::new("frb_generated");
        let old_injected_line = format!(
            "mod {}; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */"
            , path.to_str().unwrap()
        );
        let initial_content = format!(
            "pub mod api;\n{old_injected_line}\nother code;" // Old format embedded
        );
        let result_content = process_lib_rs_content(&initial_content, path).unwrap();

        let expected_code = format!(
            "{}\npub mod api;\nother code;\n", // Old format removed, new prepended, remaining code at end
            get_expected_new_block_filename_only(path)
        );
        assert_eq!(result_content, expected_code);
    }

    #[test]
    fn test_replace_only_old_format_file() {
        let path = Path::new("frb_generated");
        let old_injected_line = format!(
            "mod {}; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */"
            , path.to_str().unwrap()
        );
        let initial_content = format!("{old_injected_line}\n"); // File contains only the old format
        let result_content = process_lib_rs_content(&initial_content, path).unwrap();

        // Should be replaced by only the new block
        assert_eq!(result_content, get_expected_new_block_filename_only(path)); // Should remain unchanged
    }

    #[test]
    fn test_idempotent_after_replacement() {
        let path = Path::new("frb_generated");
        let old_injected_line = format!(
            "mod {}; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */"
            , path.to_str().unwrap()
        );
        let initial_content = format!("pub mod api;\n{old_injected_line}\n");

        let content_after_first_run = process_lib_rs_content(&initial_content, path).unwrap(); // First run: replace old with new
        let content_after_second_run =
            process_lib_rs_content(&content_after_first_run, path).unwrap(); // Second run: should not change

        assert_eq!(content_after_first_run, content_after_second_run);
        assert_eq!(
            content_after_second_run,
            get_expected_full_file_filename_only(path)
        );
    }

    #[test]
    fn test_multiple_old_formats() {
        let path = Path::new("frb_generated");

        let old_injected_line = format!(
            "mod {}; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */"
            , path.to_str().unwrap()
        );
        let initial_content = format!(
            "{old_injected_line}\npub mod api;\n{old_injected_line}\n" // Multiple old formats
        );
        let result_content = process_lib_rs_content(&initial_content, path).unwrap();
        assert_eq!(result_content, get_expected_full_file_filename_only(path));
    }

    #[test]
    fn test_multpipe_code_markers() {
        let path = Path::new("frb_generated");
        let path_str = path.to_str().unwrap();
        let initial_content = format!(
            "// AUTO INJECTED BY flutter_rust_bridge.\n\
            The following lines may not be accurate; change them according to your needs.\n\
             mod {path_str};\n\
             // this export is needed for logging\n\
             use crate::{path_str}::StreamSink as __FrbStreamSinkForLogging;\n\
             // END of AUTO INJECTED code\n\
             pub mod api;\n
             // AUTO INJECTED BY flutter_rust_bridge.\n\
            The following lines may not be accurate; change them according to your needs.\n\
             mod {path_str};\n\
             // this export is needed for logging\n\
             use crate::{path_str}::StreamSink as __FrbStreamSinkForLogging;\n\
             // END of AUTO INJECTED code\n"
        );
        let result_content = process_lib_rs_content(&initial_content, path).unwrap();
        assert_eq!(result_content, get_expected_full_file_filename_only(path));
    }

    #[test]
    #[should_panic(
        expected = "\nCould not generate code for lib.rs, as the content is wrong (missing injected code end marker '// END of AUTO INJECTED code'.\nThis is the content:\n"
    )]
    fn test_incomplete_code_markers_only_start() {
        let path = Path::new("frb_generated");
        let path_str = path.to_str().unwrap();
        let initial_content = format!(
            "// AUTO INJECTED BY flutter_rust_bridge.\n\
            The following lines may not be accurate; change them according to your needs.\n\
             mod {path_str};\n\
             // this export is needed for logging\n\
             use crate::{path_str}::StreamSink as __FrbStreamSinkForLogging;\n\
             pub mod api;\n"
        );
        let result_content = process_lib_rs_content(&initial_content, path).unwrap();
        assert_eq!(result_content, get_expected_full_file_filename_only(path));
    }

    #[test]
    #[should_panic(
        expected = "\nCould not generate code for lib.rs, as the content is wrong (missing injected code start marker '// AUTO INJECTED BY flutter_rust_bridge.'.\nThis is the content:\n"
    )]
    fn test_incomplete_code_markers_only_end() {
        let path = Path::new("frb_generated");
        let path_str = path.to_str().unwrap();
        let initial_content = format!(
            "mod {path_str};\n\
             // this export is needed for logging\n\
             use crate::{path_str}::StreamSink as __FrbStreamSinkForLogging;\n\
             // END of AUTO INJECTED code\n\
             pub mod api;\n"
        );
        let _ = process_lib_rs_content(&initial_content, path).unwrap();
    }

    #[test]
    #[should_panic(
        expected = "\nCould not generate code for lib.rs, as the content is wrong (missing injected code end marker '// END of AUTO INJECTED code'.\nThis is the content:\n"
    )]
    fn test_multiple_start_code_markers_without_end() {
        let path = Path::new("frb_generated");
        let path_str = path.to_str().unwrap();
        let initial_content = format!(
            "// AUTO INJECTED BY flutter_rust_bridge.\n\
            The following lines may not be accurate; change them according to your needs.\n\
             mod {path_str};\n\
             // this export is needed for logging\n\
             use crate::{path_str}::StreamSink as __FrbStreamSinkForLogging;\n\
             pub mod api;\n
             // AUTO INJECTED BY flutter_rust_bridge.\n\
            The following lines may not be accurate; change them according to your needs.\n\
             mod {path_str};\n\
             // this export is needed for logging\n\
             use crate::{path_str}::StreamSink as __FrbStreamSinkForLogging;\n"
        );
        let result_content = process_lib_rs_content(&initial_content, path).unwrap();
        assert_eq!(result_content, get_expected_full_file_filename_only(path));
    }
    #[test]
    #[should_panic(
        expected = "\nCould not generate code for lib.rs, as the content is wrong (missing injected code start marker '// AUTO INJECTED BY flutter_rust_bridge.'.\nThis is the content:\n"
    )]
    fn test_multiple_end_code_markers_without_start() {
        let path = Path::new("frb_generated");
        let path_str = path.to_str().unwrap();
        let initial_content = format!(
            "mod {path_str};\n\
             // this export is needed for logging\n\
             use crate::{path_str}::StreamSink as __FrbStreamSinkForLogging;\n\
             // END of AUTO INJECTED code\n\
             pub mod api;\n
             mod {path_str};\n\
             // this export is needed for logging\n\
             use crate::{path_str}::StreamSink as __FrbStreamSinkForLogging;\n\
             // END of AUTO INJECTED code\n"
        );
        let result_content = process_lib_rs_content(&initial_content, path).unwrap();
        assert_eq!(result_content, get_expected_full_file_filename_only(path));
    }

    #[test]
    fn test_new_format_old_content() {
        let path = Path::new("frb_generated");
        let path_str = path.to_str().unwrap();
        let initial_content = format!(
            "// AUTO INJECTED BY flutter_rust_bridge.\n\
            The following lines may not be accurate; change them according to your needs.\n\
             mod {path_str};\n\
             // this export is needed for logging\n\
             use crate::{path_str}::StreamSink as __FrbStreamSinkForLogging;\n\
             // END of AUTO INJECTED code\n\
             pub mod api;\n"
        );
        let result_content = process_lib_rs_content(&initial_content, path).unwrap();
        assert_eq!(result_content, get_expected_full_file_filename_only(path));
    }

    #[test]
    fn test_replace_old_generated_content_with_new() {
        let path = Path::new("frb_generated");
        let initial_content = "// AUTO INJECTED BY flutter_rust_bridge.\n\
            This are outdated lines, to be replaced!\n\
            And another old line.\n\
            If these are still inside, the test fails!\n\
             // END of AUTO INJECTED code\n\
             pub mod api;\n";
        let result_content = process_lib_rs_content(initial_content, path).unwrap();
        assert_eq!(result_content, get_expected_full_file_filename_only(path));
    }

    #[test]
    fn test_mixed_formats_and_other_content_replace_at_place() {
        let path = Path::new("frb_generated");
        let old_injected_line = format!(            "mod {}; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */"
        , path.to_str().unwrap()
    );
        let existing_new_block = get_expected_new_block_filename_only(path);

        let initial_content = format!(
            "// Some header comments\n\
             {existing_new_block}\n\
             \n\
             pub mod feature_a;\n\
             {old_injected_line}\n\n\
             pub mod feature_b;\n\n\
             {old_injected_line}\n\
             // Some footer comments\n"
        );
        let result_content = process_lib_rs_content(&initial_content, path).unwrap();

        let expected_code = format!(
            "// Some header comments\n\
             {existing_new_block}\n\
             pub mod feature_a;\n\n\
             pub mod feature_b;\n\n\
             // Some footer comments\n"
        );
        assert_eq!(result_content, expected_code);
    }

    #[test]
    fn test_only_empty_lines_differ() {
        let path = Path::new("frb_generated");
        let path_str = path.to_str().unwrap();
        let initial_content = format!(
            "// AUTO INJECTED BY flutter_rust_bridge.\n\
            // The following lines may not be accurate; change them according to your needs.\n\
            mod {path_str};\n\
            // END of AUTO INJECTED code\n\
            \n\
            // AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT\n\
            pub mod api;\n\
            mod auxiliary;\n\
            mod deliberate_name_conflict;\n\
            pub fn function_at_lib_rs()\n\
            \n"
        );
        let result_content = process_lib_rs_content(&initial_content, path).unwrap();

        assert_eq!(
            result_content.replace("\n", ""),
            initial_content.replace("\n", "")
        );
    }
    #[test]
    fn test_keep_directive_on_top() {
        let path = Path::new("frb_generated");
        let initial_content = "
            #![allow(clippy::new_without_default)]
            pub mod app;
            "
        .to_string();
        let result_content = process_lib_rs_content(&initial_content, path).unwrap();
        let expected_code = "#![allow(clippy::new_without_default)]\n\
            \n\
            // AUTO INJECTED BY flutter_rust_bridge.\n\
            // The following lines may not be accurate; change them according to your needs.\n\
            mod frb_generated;\n\
            // END of AUTO INJECTED code\n\
            \n\
            pub mod app;\n";
        assert_eq!(result_content, expected_code);
    }
    #[test]
    fn test_remove_top_newline() {
        let path = Path::new("frb_generated");
        let initial_content = "\n\
            #![allow(clippy::new_without_default)]\n\
            \n\
            // AUTO INJECTED BY flutter_rust_bridge.\n\
            // The following lines may not be accurate; change them according to your needs.\n\
            mod frb_generated;\n\
            // this export is needed for logging\n\
            use crate::frb_generated::StreamSink as __FrbStreamSinkForLogging;\n\
            // END of AUTO INJECTED code\n\
            \n\
            pub mod app;\n";
        let result_content = process_lib_rs_content(initial_content, path).unwrap();
        let expected_code = "#![allow(clippy::new_without_default)]\n\
            \n\
            // AUTO INJECTED BY flutter_rust_bridge.\n\
            // The following lines may not be accurate; change them according to your needs.\n\
            mod frb_generated;\n\
            // END of AUTO INJECTED code\n\
            \n\
            pub mod app;\n";
        assert_eq!(result_content, expected_code);
    }
}
