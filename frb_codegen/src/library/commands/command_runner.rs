use crate::utils::path_utils::{normalize_windows_unc_path, path_to_string};
use anyhow::{bail, Context};
use itertools::Itertools;
use log::debug;
use log::warn;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::process::Output;

/// - First argument is either a string of a command, or a function receiving a slice of [`PathBuf`].
///   - The command may be followed by `in <expr>` to specify the working directory.
///   - The function may be followed by an array of rest parameters to pass.
/// - Following arguments are either:
///   - An expression to turn into a [`PathBuf`]; or
///   - `?<expr>` to add `expr` only if `expr` is a [`Some`]; or
///   - `*<expr>` to concatenate an iterable of such expressions; or
///   - A tuple of `(condition, expr, ...expr)` that adds `expr`s to the arguments only if `condition` is satisfied.
///
/// Returns [`anyhow::Result<Output>`] if executing a command name, or the return value of the specified function.
#[doc(hidden)]
#[macro_export]
macro_rules! command_run {
    ($binary:ident, $($rest:tt)*) => {{
        let args = $crate::command_args!($($rest)*);
        $crate::library::commands::command_runner::execute_command($binary, args.iter(), None, None)
    }};
    ($binary:ident in $pwd:expr, envs = $envs:expr, $($rest:tt)*) => {{
        let args = $crate::command_args!($($rest)*);
        $crate::library::commands::command_runner::execute_command($binary, args.iter(), $pwd, $envs)
    }};
    ($binary:ident in $pwd:expr, $($rest:tt)*) => {{
        $crate::command_run!($binary in $pwd, envs = None, $($rest)*)
    }};
    ($command:path $([ $($args:expr),* ])?, $($rest:tt)*) => {{
        let args = $crate::command_args!($($rest)*);
        $command(&args[..] $(, $($args),* )?)
    }};
}

/// Formats a list of [`PathBuf`]s using the syntax detailed in [`command_run`].
#[doc(hidden)]
#[macro_export]
macro_rules! command_args {
    (@args $args:ident $(,)?) => {};
    (@args $args:ident ($cond:expr, $($expr:expr),+ $(,)?), $($rest:tt)*) => {
        if $cond {
            $(
                $args.push(::std::path::PathBuf::from($expr));
            )+
        }
        $crate::command_args!(@args $args $($rest)*);
    };
    (@args $args:ident ?$src:expr, $($rest:tt)*) => {
        if let Some(it) = (&$src) {
            $args.push(::std::path::PathBuf::from(it));
        }
        $crate::command_args!(@args $args $($rest)*);
    };
    (@args $args:ident *$src:expr, $($rest:tt)*) => {
        $args.extend($src.iter().map(::std::path::PathBuf::from));
        $crate::command_args!(@args $args $($rest)*);
    };
    (@args $args:ident $expr:expr, $($rest:tt)*) => {
        $args.push(::std::path::PathBuf::from($expr));
        $crate::command_args!(@args $args $($rest)*);
    };
    ($($rest:tt)*) => {{
        let mut args = Vec::new();
        $crate::command_args!(@args args $($rest)*,);
        args
    }};
}

#[allow(clippy::vec_init_then_push)]
pub(crate) fn call_shell(
    cmd: &[PathBuf],
    pwd: Option<&Path>,
    envs: Option<HashMap<String, String>>,
) -> anyhow::Result<Output> {
    let CommandInfo { program, args } = call_shell_info(cmd);
    let program = &program;
    command_run!(program in pwd, envs = envs, *args)
}

pub(crate) struct CommandInfo {
    pub program: String,
    pub args: Vec<String>,
}

pub(crate) fn call_shell_info(cmd: &[PathBuf]) -> CommandInfo {
    let cmd = cmd.iter().map(|section| format!("{section:?}")).join(" ");

    #[cfg(windows)]
    return CommandInfo {
        program: "powershell".to_owned(),
        args: vec![
            "-noprofile".to_owned(),
            "-command".to_owned(),
            format!("& {}", cmd),
        ],
    };

    #[cfg(not(windows))]
    return CommandInfo {
        program: "sh".to_owned(),
        args: vec!["-c".to_owned(), cmd],
    };
}

pub(crate) fn execute_command<'a>(
    bin: &str,
    args: impl IntoIterator<Item = &'a PathBuf>,
    current_dir: Option<&Path>,
    envs: Option<HashMap<String, String>>,
) -> anyhow::Result<Output> {
    let args = args.into_iter().collect_vec();
    let args_display = args.iter().map(|path| path.to_string_lossy()).join(" ");
    let mut cmd = Command::new(bin);
    cmd.args(args);

    if let Some(current_dir) = current_dir {
        cmd.current_dir(normalize_windows_unc_path(&path_to_string(current_dir)?));
    }
    if let Some(envs) = envs {
        cmd.envs(envs);
    }

    debug!(
        "execute command: bin={} args={:?} current_dir={:?} cmd={:?}",
        bin, args_display, current_dir, cmd
    );

    let result = cmd
        .output()
        .with_context(|| format!(r#""{bin}" "{args_display}" failed (cmd={cmd:?})"#))?;

    let stdout = String::from_utf8_lossy(&result.stdout);
    if result.status.success() {
        debug!(
            "command={:?} stdout={} stderr={}",
            cmd,
            stdout,
            String::from_utf8_lossy(&result.stderr)
        );
        if stdout.contains("fatal error") {
            // We do not care about details of this message
            // frb-coverage:ignore-start
            warn!("See keywords such as `error` in command output. Maybe there is a problem? command={:?} stdout={:?}", cmd, stdout);
            // frb-coverage:ignore-end
        }
    } else {
        warn!(
            "command={:?} stdout={} stderr={}",
            cmd,
            stdout,
            String::from_utf8_lossy(&result.stderr)
        );
    }
    Ok(result)
}

pub(crate) fn check_exit_code(res: &Output) -> anyhow::Result<()> {
    if !res.status.success() {
        // This will stop the whole generator and tell the users, so we do not care about testing it
        // frb-coverage:ignore-start
        let msg = String::from_utf8_lossy(&res.stderr);
        bail!("Command execution failed: {msg}");
        // frb-coverage:ignore-end
    }
    Ok(())
}
