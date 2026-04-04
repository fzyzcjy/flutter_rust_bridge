use crate::command_run;
use crate::commands::command_runner::call_shell;
use crate::library::commands::command_runner::check_exit_code;
use log::warn;
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

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    if should_ignore_cargo_fetch_error(&stderr) {
        warn!(
            "Skip `cargo fetch` because the manifest is nested under another workspace: {}",
            pwd.display()
        );
        return Ok(());
    }

    check_exit_code(&output)
}

fn should_ignore_cargo_fetch_error(stderr: &str) -> bool {
    stderr.contains("current package believes it's in a workspace when it's not")
}

#[cfg(test)]
mod tests {
    use super::should_ignore_cargo_fetch_error;

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
}
