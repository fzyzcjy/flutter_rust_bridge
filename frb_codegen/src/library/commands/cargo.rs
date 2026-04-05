use crate::command_run;
use crate::commands::command_runner::call_shell;
use crate::library::commands::command_runner::check_exit_code;
use log::warn;
use std::borrow::Cow;
use std::path::Path;

#[allow(clippy::vec_init_then_push)]
pub fn cargo_add(args: &[&str], pwd: &Path) -> anyhow::Result<()> {
    check_exit_code(&command_run!(
        call_shell[Some(pwd), None],
        "cargo",
        "add",
        *args,
    )?)
}

#[allow(clippy::vec_init_then_push)]
pub fn cargo_fetch(pwd: &Path) -> anyhow::Result<()> {
    let output = command_run!(
        call_shell[None, None],
        "cargo",
        "fetch",
        "--manifest-path",
        pwd.join("Cargo.toml"),
    )?;

    let stderr = String::from_utf8_lossy(&output.stderr);
    match decide_cargo_fetch_outcome(output.status.success(), &stderr) {
        CargoFetchOutcome::Success => Ok(()),
        CargoFetchOutcome::IgnoreWorkspaceConflict => {
            warn!(
                "Skip `cargo fetch` because the manifest is nested under another workspace: {}",
                pwd.display()
            );
            Ok(())
        }
        CargoFetchOutcome::Fail => check_exit_code(&output),
    }
}

#[derive(Debug, PartialEq, Eq)]
enum CargoFetchOutcome {
    Success,
    IgnoreWorkspaceConflict,
    Fail,
}

fn decide_cargo_fetch_outcome(
    status_success: bool,
    stderr: &Cow<'_, str>,
) -> CargoFetchOutcome {
    if status_success {
        return CargoFetchOutcome::Success;
    }

    if should_ignore_cargo_fetch_error(stderr) {
        return CargoFetchOutcome::IgnoreWorkspaceConflict;
    }

    CargoFetchOutcome::Fail
}

fn should_ignore_cargo_fetch_error(stderr: &str) -> bool {
    stderr.contains("current package believes it's in a workspace when it's not")
}

#[cfg(test)]
mod tests {
    use super::{
        decide_cargo_fetch_outcome, should_ignore_cargo_fetch_error, CargoFetchOutcome,
    };
    use std::borrow::Cow;

    #[test]
    fn test_should_ignore_cargo_fetch_error_workspace_conflict() {
        assert!(should_ignore_cargo_fetch_error(
            "error: current package believes it's in a workspace when it's not:"
        ));
    }

    #[test]
    fn test_should_ignore_cargo_fetch_error_other_error() {
        assert!(!should_ignore_cargo_fetch_error(
            "error: failed to download from registry"
        ));
    }

    #[test]
    fn test_decide_cargo_fetch_outcome_success() {
        assert_eq!(
            decide_cargo_fetch_outcome(true, &Cow::Borrowed("")),
            CargoFetchOutcome::Success,
        );
    }

    #[test]
    fn test_decide_cargo_fetch_outcome_workspace_conflict() {
        assert_eq!(
            decide_cargo_fetch_outcome(
                false,
                &Cow::Borrowed(
                    "error: current package believes it's in a workspace when it's not:",
                ),
            ),
            CargoFetchOutcome::IgnoreWorkspaceConflict,
        );
    }

    #[test]
    fn test_decide_cargo_fetch_outcome_other_error() {
        assert_eq!(
            decide_cargo_fetch_outcome(
                false,
                &Cow::Borrowed("error: failed to download from registry"),
            ),
            CargoFetchOutcome::Fail,
        );
    }
}
