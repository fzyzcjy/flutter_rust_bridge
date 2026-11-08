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

/// `--delete-conflicting-outputs` also deletes the outputs of other builders that the filters skip,
/// so add a filter for every generated Dart file that already exists. Falls back to an unfiltered
/// build when a file cannot be expressed as a filter or the filters would not fit on a command line.
/// The budget is a conservative cap on all systems, well below the 8191 character limit of `cmd.exe`
/// (which runs `flutter.bat`), so the rest of the command still fits.
fn include_existing_outputs(
    mut output_filters: Vec<String>,
    dart_root: &Path,
    dart_output: &Path,
    needs_json_serializable: bool,
    dart_package_name: &str,
) -> Vec<String> {
    const MAX_EXTRA_FILTER_BYTES: usize = 6_000;
    if output_filters.is_empty() {
        return output_filters;
    }
    let extra_filters = existing_generated_files(dart_root, dart_output, needs_json_serializable)
        .and_then(|files| {
            files
                .iter()
                .map(|file| {
                    let relative = path_to_string(file.strip_prefix(dart_root)?)?;
                    let relative = relative.replace(MAIN_SEPARATOR, "/");
                    build_filter_literal_path(&relative, dart_package_name)
                        .map(|path| format!("--build-filter={path}"))
                        .with_context(|| format!("Cannot filter existing output {relative:?}"))
                })
                .collect::<anyhow::Result<Vec<_>>>()
        });
    match extra_filters {
        Ok(mut filters) => {
            if filters.iter().map(|x| x.len() + 1).sum::<usize>() > MAX_EXTRA_FILTER_BYTES {
                debug!("Too many existing outputs to filter, using an unfiltered build");
                return Vec::new();
            }
            filters.sort();
            output_filters.extend(filters);
            output_filters
        }
        Err(err) => {
            debug!("Cannot filter existing outputs, using an unfiltered build: {err:?}");
            Vec::new()
        }
    }
}

/// Generated Dart files (such as `*.g.dart` of other builders) that the FRB filters do not cover.
fn existing_generated_files(
    dart_root: &Path,
    dart_output: &Path,
    needs_json_serializable: bool,
) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let mut directories = vec![dart_root.to_owned()];
    while let Some(directory) = directories.pop() {
        for entry in fs::read_dir(&directory)? {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.starts_with('.') {
                continue;
            }
            if entry.file_type()?.is_dir() {
                let is_build_dir = directory == dart_root && (name == "build" || name == "target");
                let is_cargo_target = name == "target" && directory.join("Cargo.toml").is_file();
                if name != "node_modules" && !is_build_dir && !is_cargo_target {
                    directories.push(path);
                }
            } else if !entry.file_type()?.is_file() {
                continue;
            } else if let Some((_, extension)) = name
                .strip_suffix(".dart")
                .and_then(|stem| stem.rsplit_once('.'))
            {
                let is_frb_output = directory == dart_output && name.starts_with("frb_generated.");
                let is_filtered = path.starts_with(dart_output)
                    && (extension == "freezed" || (needs_json_serializable && extension == "g"));
                if !is_frb_output && !is_filtered {
                    files.push(path);
                }
            }
        }
    }
    Ok(files)
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
        build_runner_args, build_runner_output_filters, dart_run_extra_env,
        include_existing_outputs,
    };
    use std::fs;
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

    #[test]
    fn test_include_existing_outputs() {
        let root = tempfile::tempdir().unwrap();
        for file in [
            "lib/unrelated.g.dart",
            "test/other.custom.dart",
            "lib/types [#]/example.mapper.dart",
            "lib/src/rust/api.freezed.dart",
            "lib/src/rust/api.g.dart",
            "lib/src/rust/frb_generated.io.dart",
            "lib/main.dart",
            "test/.test_config.dart",
            "build/out.g.dart",
            "target/out.g.dart",
            "node_modules/pkg/out.g.dart",
            "rust/target/debug/out.g.dart",
        ] {
            let path = root.path().join(file);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, "output").unwrap();
        }
        fs::write(root.path().join("rust/Cargo.toml"), "").unwrap();
        let frb_filter = "--build-filter=package:example/src/rust/**.freezed.dart".to_owned();
        let dart_output = root.path().join("lib/src/rust");
        let include = |filters: Vec<String>, needs_json_serializable| {
            include_existing_outputs(
                filters,
                root.path(),
                &dart_output,
                needs_json_serializable,
                "example",
            )
        };

        assert_eq!(
            include(vec![frb_filter.clone()], true),
            vec![
                frb_filter.as_str(),
                "--build-filter=package:example/types%20%5C%5B%23%5C%5D/example.mapper.dart",
                "--build-filter=package:example/unrelated.g.dart",
                "--build-filter=test/other.custom.dart",
            ]
        );
        assert!(include(vec![frb_filter.clone()], false)
            .contains(&"--build-filter=package:example/src/rust/api.g.dart".to_owned()));
        assert!(include(Vec::new(), true).is_empty());
    }

    #[test]
    fn test_include_existing_outputs_falls_back_to_unfiltered() {
        let root = tempfile::tempdir().unwrap();
        let include = || {
            include_existing_outputs(
                vec!["--build-filter=**.freezed.dart".to_owned()],
                root.path(),
                root.path(),
                false,
                "example",
            )
        };
        let test_dir = root.path().join("test");
        fs::create_dir_all(&test_dir).unwrap();

        // A path that cannot be written as a filter
        fs::write(test_dir.join("model [x].custom.dart"), "output").unwrap();
        assert!(include().is_empty());
        fs::remove_file(test_dir.join("model [x].custom.dart")).unwrap();

        // More filters than fit on a command line
        for index in 0..400 {
            fs::write(
                test_dir.join(format!("output_{index:04}.custom.dart")),
                "output",
            )
            .unwrap();
        }
        assert!(include().is_empty());

        // A directory that cannot be read
        assert!(include_existing_outputs(
            vec!["--build-filter=**.freezed.dart".to_owned()],
            &root.path().join("missing"),
            root.path(),
            false,
            "example",
        )
        .is_empty());
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
