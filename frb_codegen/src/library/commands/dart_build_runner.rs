use crate::command_args;
use crate::commands::command_runner::call_shell;
use crate::library::commands::command_runner::{call_shell_info, ExecuteCommandOptions};
use crate::library::commands::fvm::command_arg_maybe_fvm;
use crate::misc::FvmInstallMode;
use crate::utils::dart_repository::dart_repo::DartRepository;
use crate::utils::dart_repository::get_dart_package_name;
use crate::utils::path_utils::path_to_string;
use anyhow::{bail, Context};
use log::debug;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};

pub fn dart_build_runner(
    dart_root: &Path,
    dart_output: &Path,
    needs_json_serializable: bool,
    fvm_install_mode: FvmInstallMode,
) -> anyhow::Result<()> {
    debug!("Running build_runner at dart_root={dart_root:?} dart_output={dart_output:?}");

    let repo = DartRepository::from_path(dart_root)?;
    let dart_package_name = get_dart_package_name(dart_root)?;
    let output_filters = include_existing_outputs(
        build_runner_output_filters(
            dart_root,
            dart_output,
            &dart_package_name,
            needs_json_serializable,
        )?,
        dart_root,
        dart_output,
        needs_json_serializable,
        &dart_package_name,
    );
    let command_prefix = command_args!(
        ?command_arg_maybe_fvm(Some(dart_root), fvm_install_mode),
        *repo.toolchain.as_run_command(),
        *repo.command_extra_args(),
    );
    let command = build_runner_command(command_prefix, output_filters);
    let out = call_shell(
        &command,
        Some(dart_root),
        Some(ExecuteCommandOptions {
            envs: Some(dart_run_extra_env()),
            ..Default::default()
        }),
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

fn include_existing_outputs(
    mut output_filters: Vec<String>,
    dart_root: &Path,
    dart_output: &Path,
    needs_json_serializable: bool,
    dart_package_name: &str,
) -> Vec<String> {
    if output_filters.is_empty() {
        return output_filters;
    }
    let extra_filters = (|| -> anyhow::Result<Vec<String>> {
        excluded_generated_outputs(dart_root, dart_output, needs_json_serializable)?
            .into_iter()
            .map(|path| {
                let relative = path.strip_prefix(dart_root)?;
                let relative = path_to_string(relative)?.replace(MAIN_SEPARATOR, "/");
                let filter = build_filter_literal_path(&relative, dart_package_name)
                    .with_context(|| format!("Cannot filter existing output {relative:?}"))?;
                Ok(format!("--build-filter={filter}"))
            })
            .collect()
    })();
    match extra_filters {
        Ok(mut filters) => {
            filters.sort();
            output_filters.extend(filters);
            output_filters
        }
        Err(error) => {
            debug!("Cannot filter existing outputs, using an unfiltered build: {error:#}");
            Vec::new()
        }
    }
}

fn excluded_generated_outputs(
    dart_root: &Path,
    dart_output: &Path,
    needs_json_serializable: bool,
) -> anyhow::Result<Vec<PathBuf>> {
    let mut outputs = Vec::new();
    let mut directories = vec![dart_root.to_owned()];
    while let Some(directory) = directories.pop() {
        for entry in fs::read_dir(&directory)
            .with_context(|| format!("Cannot inspect build_runner outputs in {directory:?}"))?
        {
            let entry = entry?;
            let path = entry.path();
            let file_type = entry.file_type()?;
            let name = entry.file_name();
            if name.to_string_lossy().starts_with('.') {
                continue;
            }
            if file_type.is_dir() {
                if name == "node_modules"
                    || (directory == dart_root && (name == "build" || name == "target"))
                    || (name == "target" && directory.join("Cargo.toml").is_file())
                {
                    continue;
                }
                directories.push(path);
            } else if file_type.is_file() {
                let name = name.to_string_lossy();
                if directory == dart_output
                    && matches!(
                        name.as_ref(),
                        "frb_generated.io.dart" | "frb_generated.web.dart"
                    )
                {
                    continue;
                }
                let is_freezed = name.ends_with(".freezed.dart");
                let is_json = name.ends_with(".g.dart");
                let has_generated_suffix = name
                    .strip_suffix(".dart")
                    .is_some_and(|stem| stem.contains('.'));
                let covered_by_frb_filter = path.starts_with(dart_output)
                    && (is_freezed || (needs_json_serializable && is_json));
                if has_generated_suffix && !covered_by_frb_filter {
                    outputs.push(path);
                }
            }
        }
    }
    Ok(outputs)
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

fn build_runner_command(mut prefix: Vec<PathBuf>, output_filters: Vec<String>) -> Vec<PathBuf> {
    let prefix_length = prefix.len();
    prefix.extend(
        build_runner_args(output_filters)
            .into_iter()
            .map(PathBuf::from),
    );
    let shell = call_shell_info(&prefix);
    #[cfg(windows)]
    let command_length = shell.program.encode_utf16().count()
        + shell
            .args
            .iter()
            .map(|arg| 2 * arg.encode_utf16().count() + 3)
            .sum::<usize>();
    #[cfg(not(windows))]
    let command_length = shell.args.iter().map(|arg| arg.len() + 1).sum::<usize>();
    let limit = if cfg!(windows) { 30_000 } else { 64 * 1024 };
    if command_length > limit {
        debug!("Output filters exceed the shell command budget, using an unfiltered build");
        prefix.truncate(prefix_length);
        prefix.extend(build_runner_args(Vec::new()).into_iter().map(PathBuf::from));
    }
    prefix
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
    build_filter_literal_path(relative_output, dart_package_name).map(|path| format!("{path}/"))
}

fn build_filter_literal_path(relative_path: &str, dart_package_name: &str) -> Option<String> {
    if let Some(library_path) = relative_path.strip_prefix("lib/") {
        return Some(format!(
            "package:{dart_package_name}/{}",
            quote_package_glob_uri_path(library_path)
        ));
    }

    if relative_path == "lib" {
        return Some(format!("package:{dart_package_name}"));
    }

    if relative_path.bytes().all(|byte| {
        byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~' | b'/')
    }) {
        return Some(relative_path.to_owned());
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
        build_runner_args, build_runner_command, build_runner_output_filters, dart_run_extra_env,
        excluded_generated_outputs, include_existing_outputs,
    };
    use std::fs;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_build_runner_command_handles_large_filter_lists() {
        let prefix = vec![PathBuf::from("fvm"), PathBuf::from("dart")];
        let filter = "--build-filter=package:example/other.generated.dart".to_owned();
        let mut selected = prefix.clone();
        selected.extend(
            build_runner_args(vec![filter.clone()])
                .into_iter()
                .map(PathBuf::from),
        );
        assert_eq!(build_runner_command(prefix.clone(), vec![filter]), selected);

        let filters = (0..3000)
            .map(|index| {
                format!("--build-filter=package:example/serializers/output_{index}.generated.dart")
            })
            .collect();
        let mut unfiltered = prefix.clone();
        unfiltered.extend(build_runner_args(Vec::new()).into_iter().map(PathBuf::from));
        let command = build_runner_command(prefix, filters);
        assert_eq!(
            command.len(),
            unfiltered.len(),
            "Oversized filters must not reach the shell"
        );
        assert_eq!(command, unfiltered);
    }

    #[test]
    fn test_include_existing_outputs() {
        let root = tempfile::tempdir().unwrap();
        for file in [
            "lib/unrelated.g.dart",
            "test/other.custom.dart",
            "lib/types [#]/example.mapper.dart",
            "lib/src/rust/api.freezed.dart",
            "lib/src/rust/frb_generated.io.dart",
            "test/.test_config.dart",
        ] {
            let path = root.path().join(file);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, "output").unwrap();
        }
        assert_eq!(
            include_existing_outputs(
                vec!["--build-filter=package:example/src/rust/**.freezed.dart".to_owned()],
                root.path(),
                &root.path().join("lib/src/rust"),
                false,
                "example",
            ),
            vec![
                "--build-filter=package:example/src/rust/**.freezed.dart",
                "--build-filter=package:example/types%20%5C%5B%23%5C%5D/example.mapper.dart",
                "--build-filter=package:example/unrelated.g.dart",
                "--build-filter=test/other.custom.dart",
            ]
        );
        assert!(
            include_existing_outputs(Vec::new(), root.path(), root.path(), false, "example")
                .is_empty()
        );
    }

    #[test]
    fn test_include_existing_outputs_unsafe_path() {
        let root = tempfile::tempdir().unwrap();
        fs::create_dir_all(root.path().join("test")).unwrap();
        fs::write(root.path().join("test/model [x].custom.dart"), "output").unwrap();
        assert!(include_existing_outputs(
            vec!["--build-filter=**.freezed.dart".to_owned()],
            root.path(),
            root.path(),
            false,
            "example",
        )
        .is_empty());
    }

    #[test]
    fn test_existing_generated_outputs() {
        for (file, needs_json, expected) in [
            ("lib/unrelated.g.dart", false, true),
            ("lib/src/rust/frb_generated.io.dart", false, false),
            ("lib/src/rust/frb_generated.web.dart", true, false),
            ("lib/frb_generated.io.dart", true, true),
            ("lib/src/rust/nested/frb_generated.web.dart", true, true),
            ("lib/src/rust/frb_generated.custom.dart", true, true),
            ("lib/unrelated.mapper.dart", true, true),
            ("lib/src/rust/api.custom.dart", true, true),
            ("lib/src/rust/api.custom.dart", false, true),
            ("lib/unrelated.g.dart", true, true),
            ("lib/unrelated.freezed.dart", true, true),
            ("test/models/example.g.dart", true, true),
            ("lib/types [#]/example.g.dart", true, true),
            ("lib/src/rust/api.freezed.dart", false, false),
            ("lib/src/rust/api.g.dart", false, true),
            ("lib/src/rust/api.g.dart", true, false),
            ("lib/unrelated.dart", false, false),
            ("test/.test_config.dart", false, false),
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
                !excluded_generated_outputs(
                    root.path(),
                    &root.path().join("lib/src/rust"),
                    needs_json,
                )
                .unwrap()
                .is_empty(),
                expected,
                "file={file} needs_json={needs_json}",
            );
        }
    }

    #[test]
    fn test_existing_generated_outputs_at_package_root() {
        let root = tempfile::tempdir().unwrap();
        fs::write(root.path().join("example.g.dart"), "generated output").unwrap();
        assert!(!excluded_generated_outputs(root.path(), root.path(), false)
            .unwrap()
            .is_empty());
        assert!(excluded_generated_outputs(root.path(), root.path(), true)
            .unwrap()
            .is_empty());
    }

    #[test]
    fn test_existing_generated_outputs_ignore_rust_target() {
        let root = tempfile::tempdir().unwrap();
        let rust = root.path().join("native/rust");
        fs::create_dir_all(rust.join("target")).unwrap();
        fs::write(rust.join("Cargo.toml"), "").unwrap();
        fs::write(rust.join("target/example.g.dart"), "generated output").unwrap();
        assert!(excluded_generated_outputs(root.path(), root.path(), true)
            .unwrap()
            .is_empty());
    }

    #[test]
    fn test_existing_generated_outputs_propagate_read_errors() {
        let root = tempfile::tempdir().unwrap();
        let file = root.path().join("not_a_directory");
        fs::write(&file, "").unwrap();
        assert!(excluded_generated_outputs(&file, &file, true).is_err());
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

    /// Uses the full build when inspecting existing outputs fails.
    #[test]
    fn test_clear_filters_falls_back_on_inspection_error() {
        let temp = tempfile::tempdir().unwrap();
        let missing = temp.path().join("missing");
        assert_eq!(
            include_existing_outputs(
                vec!["--build-filter=package:example/src/rust/**.freezed.dart".to_owned()],
                &missing,
                &missing.join("lib/src/rust"),
                false,
                "example",
            ),
            Vec::<String>::new()
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
