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
use std::path::{Path, MAIN_SEPARATOR};

pub fn dart_build_runner(
    dart_root: &Path,
    dart_output: &Path,
    needs_json_serializable: bool,
    fvm_install_mode: FvmInstallMode,
) -> anyhow::Result<()> {
    debug!("Running build_runner at dart_root={dart_root:?} dart_output={dart_output:?}");

    let repo = DartRepository::from_path(dart_root)?;
    let output_filters = build_runner_output_filters(
        dart_root,
        dart_output,
        &get_dart_package_name(dart_root)?,
        needs_json_serializable,
    )?;
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
            "Failed to run build_runner for {:?}: {}",
            dart_root,
            String::from_utf8_lossy(&out.stdout)
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
    use super::{build_runner_args, build_runner_output_filters};
    use std::path::Path;

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
}
