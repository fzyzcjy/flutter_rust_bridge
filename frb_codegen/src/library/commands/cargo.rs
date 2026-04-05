use crate::command_run;
use crate::commands::command_runner::call_shell;
use crate::library::commands::command_runner::check_exit_code;
use anyhow::Result;
use log::warn;
use std::borrow::Cow;
use std::path::Path;
use std::process::Output;

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
pub fn cargo_fetch(pwd: &Path) -> Result<()> {
    // the command macro body is covered via wrapper tests, but llvm-cov still reports
    // the macro-closing line as uncovered
    // frb-coverage:ignore-start
    cargo_fetch_with(pwd, |manifest_path| {
        Ok(command_run!(
            call_shell[None, None],
            "cargo",
            "fetch",
            "--manifest-path",
            manifest_path,
        )?)
    })
    // frb-coverage:ignore-end
}

fn cargo_fetch_with(
    pwd: &Path,
    run_fetch: impl FnOnce(&Path) -> Result<Output>,
) -> Result<()> {
    let output = run_fetch(&pwd.join("Cargo.toml"))?;
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
        cargo_fetch_with, decide_cargo_fetch_outcome, should_ignore_cargo_fetch_error,
        CargoFetchOutcome,
    };
    use anyhow::Result;
    use std::borrow::Cow;
    use std::fs;
    use std::path::Path;
    use std::process::Command;
    use std::process::Output;

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

    #[test]
    fn test_cargo_fetch_with_success() {
        cargo_fetch_with(Path::new("/tmp/project"), |_| {
            Ok(fake_output(0, ""))
        })
        .unwrap();
    }

    #[test]
    fn test_cargo_fetch_with_workspace_conflict() {
        cargo_fetch_with(Path::new("/tmp/project"), |_| {
            Ok(fake_output(
                1,
                "error: current package believes it's in a workspace when it's not:",
            ))
        })
        .unwrap();
    }

    #[test]
    fn test_cargo_fetch_with_other_error() {
        let error = cargo_fetch_with(Path::new("/tmp/project"), |_| {
            Ok(fake_output(1, "error: failed to download from registry"))
        })
        .unwrap_err();

        assert!(
            error
                .to_string()
                .contains("failed to download from registry"),
        );
    }

    #[test]
    fn test_cargo_fetch_success_for_standalone_manifest() {
        let temp_dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(temp_dir.path().join("src")).unwrap();
        fs::write(
            temp_dir.path().join("Cargo.toml"),
            "[package]\nname = \"cargo_fetch_test\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        )
        .unwrap();
        fs::write(temp_dir.path().join("src/lib.rs"), "pub fn answer() -> i32 { 42 }\n")
            .unwrap();

        super::cargo_fetch(temp_dir.path()).unwrap();
    }

    fn fake_output(exit_code: i32, stderr: &str) -> Output {
        run_shell(exit_code, stderr).unwrap()
    }

    #[cfg(unix)]
    fn run_shell(exit_code: i32, stderr: &str) -> Result<Output> {
        Command::new("sh")
            .arg("-c")
            .arg("printf '%s' \"$1\" >&2; exit \"$2\"")
            .arg("sh")
            .arg(stderr)
            .arg(exit_code.to_string())
            .output()
            .map_err(Into::into)
    }

    #[cfg(windows)]
    fn run_shell(exit_code: i32, stderr: &str) -> Result<Output> {
        Command::new("cmd")
            .args([
                "/C",
                &format!("echo {stderr} 1>&2 & exit /b {exit_code}"),
            ])
            .output()
            .map_err(Into::into)
    }
}
