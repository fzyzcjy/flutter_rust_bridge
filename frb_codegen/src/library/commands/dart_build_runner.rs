use crate::command_run;
use crate::commands::command_runner::call_shell;
use crate::library::commands::command_runner::ExecuteCommandOptions;
use crate::library::commands::fvm::command_arg_maybe_fvm;
use crate::misc::FvmInstallMode;
use crate::utils::dart_repository::dart_repo::DartRepository;
use crate::utils::path_utils::path_to_string;
use anyhow::{bail, Context};
use log::debug;
use std::collections::HashMap;
use std::path::Path;

pub fn dart_build_runner(
    dart_root: &Path,
    dart_output: &Path,
    fvm_install_mode: FvmInstallMode,
) -> anyhow::Result<()> {
    debug!("Running build_runner at dart_root={dart_root:?} dart_output={dart_output:?}");

    let repo = DartRepository::from_path(dart_root)?;
    let output_filters = build_runner_output_filters(dart_root, dart_output)?;
    let out = command_run!(
        call_shell[Some(dart_root), Some(ExecuteCommandOptions {
            envs: Some(dart_run_extra_env()),
            ..Default::default()
        })],
        ?command_arg_maybe_fvm(Some(dart_root), fvm_install_mode),
        *repo.toolchain.as_run_command(),
        *repo.command_extra_args(),
        "run",
        "build_runner",
        "build",
        "--delete-conflicting-outputs",
        *output_filters,
        "--enable-experiment=class-modifiers",
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
) -> anyhow::Result<Vec<String>> {
    let relative_output = dart_output.strip_prefix(dart_root).with_context(|| {
        format!("dart_output={dart_output:?} must be within dart_root={dart_root:?}")
    })?;
    let relative_output = path_to_string(relative_output)?.replace('\\', "/");
    let output_prefix = if relative_output.is_empty() {
        String::new()
    } else {
        format!("{relative_output}/")
    };

    Ok(["freezed.dart", "g.dart"]
        .map(|extension| format!("--build-filter={output_prefix}**.{extension}"))
        .to_vec())
}

#[cfg(test)]
mod tests {
    use super::build_runner_output_filters;
    use std::path::Path;

    /// Limits build runner to generated outputs beneath a nested Dart output directory.
    #[test]
    fn test_build_runner_output_filters_nested_output() {
        assert_eq!(
            build_runner_output_filters(Path::new("/project"), Path::new("/project/lib/src/rust"),)
                .unwrap(),
            vec![
                "--build-filter=lib/src/rust/**.freezed.dart",
                "--build-filter=lib/src/rust/**.g.dart",
            ]
        );
    }

    /// Supports a Dart output directory at the package root without an absolute filter.
    #[test]
    fn test_build_runner_output_filters_root_output() {
        assert_eq!(
            build_runner_output_filters(Path::new("/project"), Path::new("/project")).unwrap(),
            vec!["--build-filter=**.freezed.dart", "--build-filter=**.g.dart"]
        );
    }

    /// Rejects output directories that build runner cannot address from the Dart package.
    #[test]
    fn test_build_runner_output_filters_outside_dart_root() {
        let error = build_runner_output_filters(
            Path::new("/project/dart"),
            Path::new("/project/generated"),
        )
        .unwrap_err();

        assert!(error.to_string().contains("must be within dart_root"));
    }
}
