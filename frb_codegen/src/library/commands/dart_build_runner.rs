use crate::command_run;
use crate::commands::command_runner::call_shell;
use crate::library::commands::command_runner::ExecuteCommandOptions;
use crate::library::commands::fvm::command_arg_maybe_fvm;
use crate::misc::FvmInstallMode;
use crate::utils::dart_repository::dart_repo::DartRepository;
use crate::utils::dart_repository::get_dart_package_name;
use crate::utils::path_utils::path_to_string;
use anyhow::{bail, Context};
use log::debug;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, MAIN_SEPARATOR};

pub fn dart_build_runner(
    dart_root: &Path,
    dart_output: &Path,
    needs_json_serializable: bool,
    fvm_install_mode: FvmInstallMode,
) -> anyhow::Result<()> {
    debug!("Running build_runner at dart_root={dart_root:?} dart_output={dart_output:?}");

    let repo = DartRepository::from_path(dart_root)?;
    let mut output_filters = build_runner_output_filters(
        dart_root,
        dart_output,
        &get_dart_package_name(dart_root)?,
        needs_json_serializable,
    )?;
    if !output_filters.is_empty()
        && has_excluded_generated_outputs(dart_root, dart_output, needs_json_serializable)?
    {
        debug!("Running unfiltered build_runner to rebuild existing excluded outputs");
        output_filters.clear();
    }
    let args = build_runner_args(output_filters);
    let out = command_run!(
        call_shell[Some(dart_root), Some(ExecuteCommandOptions {
            envs: Some(dart_run_extra_env()),
            ..Default::default()
        })],
        ?command_arg_maybe_fvm(Some(dart_root), fvm_install_mode),
        *repo.toolchain.as_run_command(),
        *repo.command_extra_args(),
        *args,
    )?;
    if !out.status.success() {
        // This will stop the whole generator and tell the users, so we do not care about testing it
        // frb-coverage:ignore-start
        bail!(
            "Failed to run build_runner for {:?}: {}\n{}",
            dart_root,
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        // frb-coverage:ignore-end
    }
    Ok(())
}

pub(super) fn dart_run_extra_env() -> HashMap<String, String> {
    // Hack before https://github.com/dart-lang/native/issues/822 is fixed
    // Otherwise every call to `ffigen`, `build_runner`, etc will need to
    // trigger `build.dart`, which takes minutes to compile the `./rust` crate
    [("FRB_SIMPLE_BUILD_SKIP".to_owned(), "1".to_owned())].into()
}

fn has_excluded_generated_outputs(
    dart_root: &Path,
    dart_output: &Path,
    needs_json_serializable: bool,
) -> anyhow::Result<bool> {
    let mut directories = vec![dart_root.to_owned()];
    while let Some(directory) = directories.pop() {
        for entry in fs::read_dir(&directory)
            .with_context(|| format!("Cannot inspect build_runner outputs in {directory:?}"))?
        {
            let entry = entry?;
            let path = entry.path();
            let file_type = entry.file_type()?;
            let name = entry.file_name();
            if file_type.is_dir() {
                if name.to_string_lossy().starts_with('.')
                    || name == "node_modules"
                    || (directory == dart_root && (name == "build" || name == "target"))
                    || (name == "target" && directory.join("Cargo.toml").is_file())
                {
                    continue;
                }
                directories.push(path);
            } else if file_type.is_file() {
                let name = name.to_string_lossy();
                let is_freezed = name.ends_with(".freezed.dart");
                let is_json = name.ends_with(".g.dart");
                if (is_freezed || is_json)
                    && !(path.starts_with(dart_output) && (is_freezed || needs_json_serializable))
                {
                    return Ok(true);
                }
            }
        }
    }
    Ok(false)
}

fn build_runner_output_filters(
    dart_root: &Path,
    dart_output: &Path,
    dart_package_name: &str,
    needs_json_serializable: bool,
) -> anyhow::Result<Vec<String>> {
    let relative_output = dart_output.strip_prefix(dart_root).with_context(|| {
        format!("dart_output={dart_output:?} must be within dart_root={dart_root:?}")
    })?;
    let relative_output = path_to_string(relative_output)?.replace(MAIN_SEPARATOR, "/");
    let Some(output_prefix) = build_filter_output_prefix(&relative_output, dart_package_name)
    else {
        debug!("Falling back to unfiltered build_runner for dart_output path {relative_output:?}");
        return Ok(Vec::new());
    };

    let mut extensions = vec!["freezed.dart"];
    if needs_json_serializable {
        extensions.push("g.dart");
    }

    Ok(extensions
        .into_iter()
        .map(|extension| format!("--build-filter={output_prefix}**.{extension}"))
        .collect())
}

fn build_runner_args(output_filters: Vec<String>) -> Vec<String> {
    [
        vec![
            "run".to_owned(),
            "build_runner".to_owned(),
            "build".to_owned(),
            "--delete-conflicting-outputs".to_owned(),
        ],
        output_filters,
    ]
    .concat()
}

fn build_filter_output_prefix(relative_output: &str, dart_package_name: &str) -> Option<String> {
    if relative_output.is_empty() {
        return Some(String::new());
    }

    if let Some(library_output) = relative_output.strip_prefix("lib/") {
        return Some(format!(
            "package:{dart_package_name}/{}/",
            quote_package_glob_uri_path(library_output)
        ));
    }

    if relative_output == "lib" {
        return Some(format!("package:{dart_package_name}/"));
    }

    if relative_output.bytes().all(|byte| {
        byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~' | b'/')
    }) {
        return Some(format!("{relative_output}/"));
    }

    None
}

fn quote_package_glob_uri_path(path: &str) -> String {
    let mut quoted = String::with_capacity(path.len());
    for character in path.chars() {
        if matches!(
            character,
            '*' | '{' | '[' | '?' | '\\' | '}' | ']' | ',' | '-' | '(' | ')'
        ) {
            quoted.push('\\');
        }
        quoted.push(character);
    }

    percent_encode_uri_path(&quoted)
}

fn percent_encode_uri_path(path: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";

    let mut encoded = String::with_capacity(path.len());
    for byte in path.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~' | b'/') {
            encoded.push(byte as char);
        } else {
            encoded.push('%');
            encoded.push(HEX[(byte >> 4) as usize] as char);
            encoded.push(HEX[(byte & 0x0f) as usize] as char);
        }
    }
    encoded
}

#[cfg(test)]
mod tests {
    use super::{
        build_runner_args, build_runner_output_filters, dart_run_extra_env,
        has_excluded_generated_outputs,
    };
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_existing_generated_outputs() {
        for (file, needs_json, expected) in [
            ("lib/unrelated.g.dart", false, true),
            ("lib/unrelated.g.dart", true, true),
            ("lib/unrelated.freezed.dart", true, true),
            ("test/models/example.g.dart", true, true),
            ("lib/types [#]/example.g.dart", true, true),
            ("lib/src/rust/api.freezed.dart", false, false),
            ("lib/src/rust/api.g.dart", false, true),
            ("lib/src/rust/api.g.dart", true, false),
            ("lib/unrelated.dart", false, false),
            ("lib/build/example.g.dart", true, true),
            (".dart_tool/build/example.g.dart", true, false),
            ("build/example.g.dart", true, false),
            ("target/example.g.dart", true, false),
            ("node_modules/example.g.dart", true, false),
        ] {
            let root = tempfile::tempdir().unwrap();
            let path = root.path().join(file);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, "generated output").unwrap();
            assert_eq!(
                has_excluded_generated_outputs(
                    root.path(),
                    &root.path().join("lib/src/rust"),
                    needs_json,
                )
                .unwrap(),
                expected,
                "file={file} needs_json={needs_json}",
            );
        }
    }

    #[test]
    fn test_existing_generated_outputs_at_package_root() {
        let root = tempfile::tempdir().unwrap();
        fs::write(root.path().join("example.g.dart"), "generated output").unwrap();
        assert!(has_excluded_generated_outputs(root.path(), root.path(), false).unwrap());
        assert!(!has_excluded_generated_outputs(root.path(), root.path(), true).unwrap());
    }

    #[test]
    fn test_existing_generated_outputs_ignore_rust_target() {
        let root = tempfile::tempdir().unwrap();
        let rust = root.path().join("native/rust");
        fs::create_dir_all(rust.join("target")).unwrap();
        fs::write(rust.join("Cargo.toml"), "").unwrap();
        fs::write(rust.join("target/example.g.dart"), "generated output").unwrap();
        assert!(!has_excluded_generated_outputs(root.path(), root.path(), true).unwrap());
    }

    #[test]
    fn test_existing_generated_outputs_propagate_read_errors() {
        let root = tempfile::tempdir().unwrap();
        let file = root.path().join("not_a_directory");
        fs::write(&file, "").unwrap();
        assert!(has_excluded_generated_outputs(&file, &file, true).is_err());
    }

    /// Limits build runner to generated outputs beneath a nested Dart output directory.
    #[test]
    fn test_build_runner_output_filters_nested_output() {
        assert_eq!(
            build_runner_output_filters(
                Path::new("/project"),
                Path::new("/project/lib/src/rust"),
                "example",
                true,
            )
            .unwrap(),
            vec![
                "--build-filter=package:example/src/rust/**.freezed.dart",
                "--build-filter=package:example/src/rust/**.g.dart",
            ]
        );
    }

    /// Includes JSON serialization output only when the generator requires it.
    #[test]
    fn test_build_runner_output_filters_without_json_serializable() {
        assert_eq!(
            build_runner_output_filters(
                Path::new("/project"),
                Path::new("/project/lib/src/rust"),
                "example",
                false,
            )
            .unwrap(),
            vec!["--build-filter=package:example/src/rust/**.freezed.dart"]
        );
    }

    /// Supports a Dart output directory at the package root without an absolute filter.
    #[test]
    fn test_build_runner_output_filters_root_output() {
        assert_eq!(
            build_runner_output_filters(
                Path::new("/project"),
                Path::new("/project"),
                "example",
                false,
            )
            .unwrap(),
            vec!["--build-filter=**.freezed.dart"]
        );
    }

    /// Escapes glob metacharacters and URI delimiters in the literal output directory.
    #[test]
    fn test_build_runner_output_filters_metacharacter_output() {
        assert_eq!(
            build_runner_output_filters(
                Path::new("/project"),
                Path::new("/project/lib/generated[foo] #bar?"),
                "example",
                false,
            )
            .unwrap(),
            vec!["--build-filter=package:example/generated%5C%5Bfoo%5C%5D%20%23bar%5C%3F/**.freezed.dart"]
        );
    }

    /// Rejects output directories that build runner cannot address from the Dart package.
    #[test]
    fn test_build_runner_output_filters_outside_dart_root() {
        let error = build_runner_output_filters(
            Path::new("/project/dart"),
            Path::new("/project/generated"),
            "example",
            false,
        )
        .unwrap_err();

        assert!(error.to_string().contains("must be within dart_root"));
    }

    /// Falls back to an unfiltered build when a non-library output path cannot be filtered safely.
    #[test]
    fn test_build_runner_output_filters_metacharacter_output_outside_lib() {
        assert_eq!(
            build_runner_output_filters(
                Path::new("/project"),
                Path::new("/project/generated[foo]"),
                "example",
                false,
            )
            .unwrap(),
            Vec::<String>::new()
        );
    }

    /// Filters a URI-safe output path outside the package library.
    #[test]
    fn test_build_runner_output_filters_safe_output_outside_lib() {
        assert_eq!(
            build_runner_output_filters(
                Path::new("/project"),
                Path::new("/project/test/generated"),
                "example",
                false,
            )
            .unwrap(),
            vec!["--build-filter=test/generated/**.freezed.dart"]
        );
    }

    /// Constructs a build_runner command supported by the declared minimum version.
    #[test]
    fn test_build_runner_args() {
        assert_eq!(
            build_runner_args(vec!["--build-filter=lib/**.freezed.dart".to_owned()]),
            vec![
                "run",
                "build_runner",
                "build",
                "--delete-conflicting-outputs",
                "--build-filter=lib/**.freezed.dart",
            ]
        );
    }

    /// Sets the build-script bypass required by every Dart subprocess.
    #[test]
    fn test_dart_run_extra_env() {
        assert_eq!(
            dart_run_extra_env().get("FRB_SIMPLE_BUILD_SKIP"),
            Some(&"1".to_owned())
        );
    }
}
