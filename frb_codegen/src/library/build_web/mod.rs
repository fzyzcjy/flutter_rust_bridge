//! Build web platform for a Flutter+Rust app

use crate::command_run;
use crate::library::commands::command_runner::{call_shell, call_shell_info, check_exit_code};
use crate::utils::dart_repository::dart_repo::DartRepository;
use crate::utils::path_utils::{find_dart_package_dir, path_to_string};
use anyhow::{bail, Context};
use itertools::Itertools;
use log::debug;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use std::str::FromStr;
use std::{env, fs};

// We make the core build-web logic in Dart, and Rust is just a wrapper.
// This is because, in the future, the build-web logic may be packaged with user libraries
// and invoked in machines without flutter_rust_bridge_codegen binary.
pub fn build(
    dart_root: Option<PathBuf>,
    dart_coverage: bool,
    args: Vec<String>,
) -> anyhow::Result<()> {
    let dart_root = parse_dart_root(dart_root)?;
    debug!("build dart_root={dart_root:?} args={args:?}");
    execute_dart_command(&dart_root, &args, dart_coverage)
}

fn parse_dart_root(dart_root: Option<PathBuf>) -> anyhow::Result<PathBuf> {
    dart_root
        .map(|x| Ok(fs::canonicalize(x)?))
        .unwrap_or_else(|| {
            find_dart_package_dir(&env::current_dir()?)
                .context("Please provide --dart-root, or run command inside a Flutter/Dart package")
        })
}

fn execute_dart_command(
    dart_root: &Path,
    args: &[String],
    dart_coverage: bool,
) -> anyhow::Result<()> {
    let repo = DartRepository::from_str(&path_to_string(dart_root)?)?;

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
    let status = dart_run(&repo, dart_root, dart_coverage, dart_run_args)?;

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
) -> anyhow::Result<ExitStatus> {
    let handle = {
        let mut cmd_args: Vec<PathBuf> = vec!["dart".into()];
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

        let info = call_shell_info(&cmd_args);
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
            // TODO this scope-output?
            "--scope-output=foo",
        )?;
        check_exit_code(&res)?;
    }

    Ok(handle.wait_with_output()?.status)
}
