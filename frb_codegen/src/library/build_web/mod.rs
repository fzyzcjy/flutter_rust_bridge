use crate::command_run;
use crate::commands::command_runner::call_shell;
use crate::library::commands::command_runner::execute_command;
use crate::utils::dart_repository::dart_repo::DartRepository;
use crate::utils::path_utils::{find_dart_package_dir, path_to_string};
use anyhow::{bail, Context};
use log::debug;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{env, fs};

pub fn build(dart_root: Option<PathBuf>, args: Vec<String>) -> anyhow::Result<()> {
    let dart_root = parse_dart_root(dart_root)?;
    debug!("build dart_root={dart_root:?} args={args:?}");
    execute_dart_command(&dart_root, &args)
}

fn parse_dart_root(dart_root: Option<PathBuf>) -> anyhow::Result<PathBuf> {
    dart_root
        .map(|x| Ok(fs::canonicalize(x)?))
        .unwrap_or_else(|| {
            find_dart_package_dir(&env::current_dir()?)
                .context("Please provide --dart-root, or run command inside a Flutter/Dart package")
        })
}

fn execute_dart_command(dart_root: &Path, args: &[String]) -> anyhow::Result<()> {
    let repo = DartRepository::from_str(&path_to_string(dart_root)?)?;

    let res = command_run!(
        call_shell[Some(dart_root)],
        *repo.toolchain.as_run_command(),
        *args,
    )?;

    if !res.status.success() {
        bail!(
            "Fail to execute command: {}",
            String::from_utf8_lossy(&res.stderr),
        )
    }
    Ok(())
}
