use crate::command_run;
use crate::commands::command_runner::call_shell;
use crate::utils::dart_repository::dart_repo::DartRepository;
use crate::utils::path_utils::path_to_string;
use anyhow::bail;
use log::debug;
use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;

pub fn dart_build_runner(dart_root: &Path) -> anyhow::Result<()> {
    debug!("Running build_runner at dart_root={dart_root:?}");

    let repo = DartRepository::from_str(&path_to_string(dart_root)?).unwrap();
    let out = command_run!(
        call_shell[Some(dart_root), Some(dart_run_extra_env())],
        *repo.toolchain.as_run_command(),
        *repo.command_extra_args(),
        "run",
        "build_runner",
        "build",
        "--delete-conflicting-outputs",
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
