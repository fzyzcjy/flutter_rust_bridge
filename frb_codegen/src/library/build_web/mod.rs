//! Build web platform for a Flutter+Rust app

use crate::command_run;
use crate::library::commands::command_runner::{call_shell, call_shell_info, check_exit_code};
use crate::library::commands::fvm::command_arg_maybe_fvm;
use crate::misc::FvmInstallMode;
use crate::utils::dart_repository::dart_repo::DartRepository;
use crate::utils::path_utils::{find_dart_package_dir, path_to_string};
use anyhow::{bail, Context};
use itertools::Itertools;
use log::debug;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use std::{env, fs};

// We make the core build-web logic in Dart, and Rust is just a wrapper.
// This is because, in the future, the build-web logic may be packaged with user libraries
// and invoked in machines without flutter_rust_bridge_codegen binary.
pub fn build(
    dart_root: Option<PathBuf>,
    dart_coverage: bool,
    args: Vec<String>,
    fvm_install_mode: FvmInstallMode,
) -> anyhow::Result<()> {
    let dart_root = parse_dart_root(dart_root)?;
    debug!("build dart_root={dart_root:?} args={args:?}");
    execute_dart_command(&dart_root, &args, dart_coverage, fvm_install_mode)
}

fn parse_dart_root(dart_root: Option<PathBuf>) -> anyhow::Result<PathBuf> {
    dart_root
        .map(|x| Ok(fs::canonicalize(x)?))
        .unwrap_or_else(|| parse_discovered_dart_root(&env::current_dir()?))
}

fn parse_discovered_dart_root(current_dir: &Path) -> anyhow::Result<PathBuf> {
    find_dart_package_dir(current_dir)
        .context("Please provide --dart-root, or run command inside a Flutter/Dart package")
}

fn execute_dart_command(
    dart_root: &Path,
    args: &[String],
    dart_coverage: bool,
    fvm_install_mode: FvmInstallMode,
) -> anyhow::Result<()> {
    let repo = DartRepository::from_path(dart_root)?;

    let dart_run_args = {
        let mut ans = vec![
            "flutter_rust_bridge".to_owned(),
            "build-web".to_owned(),
            "--dart-root".to_owned(),
            path_to_string(dart_root)?,
        ];
        ans.extend(args.to_owned());
        ans
    };
    let status = dart_run(
        &repo,
        dart_root,
        dart_coverage,
        dart_run_args,
        fvm_install_mode,
    )?;

    if !status.success() {
        // This will stop the whole generator and tell the users, so we do not care about testing it
        // frb-coverage:ignore-start
        bail!("Fail to execute command, please see logs above for details.")
        // frb-coverage:ignore-end
    }

    Ok(())
}

// ref: https://pub.dev/packages/coverage
#[allow(clippy::vec_init_then_push)]
fn dart_run(
    repo: &DartRepository,
    current_dir: &Path,
    dart_coverage: bool,
    args: Vec<String>,
    fvm_install_mode: FvmInstallMode,
) -> anyhow::Result<ExitStatus> {
    let handle = {
        let cmd_args = dart_run_command_args(repo, dart_coverage, args, fvm_install_mode);

        let info = call_shell_info(&cmd_args)?;
        Command::new(info.program)
            .args(info.args)
            .current_dir(current_dir)
            .spawn()?
    };

    if dart_coverage {
        let res = command_run!(
            call_shell[Some(current_dir), None],
            "dart",
            "pub",
            "global",
            "run",
            "coverage:collect_coverage",
            "--wait-paused",
            "--uri=http://127.0.0.1:8181/",
            "-o",
            "coverage/coverage.json",
            "--resume-isolates",
        )?;
        check_exit_code(&res)?;
    }

    Ok(handle.wait_with_output()?.status)
}

fn dart_run_command_args(
    repo: &DartRepository,
    dart_coverage: bool,
    args: Vec<String>,
    fvm_install_mode: FvmInstallMode,
) -> Vec<PathBuf> {
    let cmd_args: Vec<PathBuf> = if command_arg_maybe_fvm(None, fvm_install_mode).is_some() {
        vec!["fvm".into(), "dart".into()]
    } else {
        vec!["dart".into()]
    };
    append_dart_run_command_args(cmd_args, repo, dart_coverage, args)
}

fn append_dart_run_command_args(
    mut cmd_args: Vec<PathBuf>,
    repo: &DartRepository,
    dart_coverage: bool,
    args: Vec<String>,
) -> Vec<PathBuf> {
    cmd_args.extend(repo.command_extra_args().into_iter().map_into());
    cmd_args.push("run".into());
    if dart_coverage {
        cmd_args.extend([
            "--pause-isolates-on-exit".into(),
            "--disable-service-auth-codes".into(),
            "--enable-vm-service=8181".into(),
        ]);
    }
    cmd_args.extend(args.into_iter().map_into());
    cmd_args
}

#[cfg(test)]
mod tests {
    use super::{append_dart_run_command_args, parse_dart_root, parse_discovered_dart_root};
    use crate::library::commands::command_runner::call_shell_info;
    use crate::utils::dart_repository::dart_repo::DartRepository;
    use anyhow::Result;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn dart_repository(temp_dir: &tempfile::TempDir) -> Result<DartRepository> {
        fs::write(temp_dir.path().join("pubspec.yaml"), "name: test_package\n")?;
        DartRepository::from_path(temp_dir.path())
    }

    /// Canonicalizes an explicitly supplied Dart package root.
    #[test]
    fn test_parse_dart_root_uses_explicit_root() -> Result<()> {
        let temp_dir = tempdir()?;
        fs::create_dir(temp_dir.path().join("package"))?;
        let root = temp_dir.path().join("package");

        assert_eq!(parse_dart_root(Some(root.clone()))?, root.canonicalize()?);
        Ok(())
    }

    /// Discovers the nearest Dart package root from an injected current directory.
    #[test]
    fn test_parse_dart_root_discovers_current_package() -> Result<()> {
        let temp_dir = tempdir()?;
        let package = temp_dir.path().join("package");
        let nested = package.join("lib/src");
        fs::create_dir_all(&nested)?;
        fs::write(package.join("pubspec.yaml"), "name: test_package\n")?;

        assert_eq!(parse_discovered_dart_root(&nested)?, package);
        Ok(())
    }

    /// Builds a shell-safe non-coverage Dart invocation without starting Dart.
    #[test]
    fn test_dart_run_command_args_without_coverage() -> Result<()> {
        let temp_dir = tempdir()?;
        let repo = dart_repository(&temp_dir)?;
        let args = append_dart_run_command_args(
            vec![PathBuf::from("dart")],
            &repo,
            false,
            vec!["flutter_rust_bridge".to_owned(), "build-web".to_owned()],
        );

        assert_eq!(
            args,
            vec!["dart", "run", "flutter_rust_bridge", "build-web"]
                .into_iter()
                .map(PathBuf::from)
                .collect::<Vec<_>>(),
        );
        let info = call_shell_info(&args)?;
        assert_eq!(
            info.program,
            if cfg!(windows) { "powershell" } else { "sh" }
        );
        Ok(())
    }

    /// Adds all VM-service flags to the coverage invocation without starting Dart.
    #[test]
    fn test_dart_run_command_args_with_coverage() -> Result<()> {
        let temp_dir = tempdir()?;
        let repo = dart_repository(&temp_dir)?;
        let args = append_dart_run_command_args(
            vec![PathBuf::from("dart")],
            &repo,
            true,
            vec!["flutter_rust_bridge".to_owned(), "build-web".to_owned()],
        );

        assert_eq!(
            args,
            vec![
                "dart",
                "run",
                "--pause-isolates-on-exit",
                "--disable-service-auth-codes",
                "--enable-vm-service=8181",
                "flutter_rust_bridge",
                "build-web",
            ]
            .into_iter()
            .map(PathBuf::from)
            .collect::<Vec<_>>(),
        );
        let info = call_shell_info(&args)?;
        assert!(!info.args.is_empty());
        Ok(())
    }
}
