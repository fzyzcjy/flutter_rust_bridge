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
    ($binary:ident in $pwd:expr, options = $options:expr, $($rest:tt)*) => {{
        let args = $crate::command_args!($($rest)*);
        $crate::library::commands::command_runner::execute_command($binary, args.iter(), $pwd, $options)
    }};
    ($binary:ident in $pwd:expr, $($rest:tt)*) => {{
        $crate::command_run!($binary in $pwd, options = None, $($rest)*)
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
    options: Option<ExecuteCommandOptions>,
) -> anyhow::Result<Output> {
    let CommandInfo { program, args } = call_shell_info(cmd);
    let program = &program;
    command_run!(program in pwd, options = options, *args)
}

#[derive(Debug, PartialEq)]
pub(crate) struct CommandInfo {
    pub program: String,
    pub args: Vec<String>,
}

pub(crate) fn call_shell_info(cmd: &[PathBuf]) -> CommandInfo {
    #[cfg(windows)]
    {
        let cmd = cmd
            .iter()
            .map(|section| windows_escape_for_powershell(section.to_str().unwrap()))
            .join(" ");
        return CommandInfo {
            program: "powershell".to_owned(),
            args: vec![
                "-noprofile".to_owned(),
                "-command".to_owned(),
                format!("& {}", cmd),
            ],
        };
    }

    #[cfg(not(windows))]
    {
        let cmd = cmd.iter().map(|section| format!("{section:?}")).join(" ");
        return CommandInfo {
            program: "sh".to_owned(),
            args: vec!["-c".to_owned(), cmd],
        };
    }
}

/// Applies a minimal set of backtick escapes to convert a string into a PowerShell 5.1 argument token.
///
/// Note: The escapes are for PowerShell 5.1 or earlier (`powershell.exe`) which is invoked
/// the by the calling call_shell_info() function, not PowerShell 7+ (`pwsh.exe`).
///
/// PowerShell 5.1 (i.e. `powershell.exe` on Windows), has particular string
/// escaping requirements. This function handles escaping of special characters to ensure the
/// string is passed as a single, intact argument token. The PowerShell 5.1 argument-mode metacharacters
/// (characters with special syntactic meaning) are:
///
///   \: File path separator (e.g., C:\Users) and escape character in some contexts.
///   &: Begins argument mode and background execution.
///   *: Wildcard for filename expansion (globbing), matches zero or more characters in file paths (e.g., *.txt).
///   +: Used for string concatenation.
///   ?: Wildcard matching a single character in paths (e.g., file?.txt matches file1.txt).
///   |: Pipeline operator; sends output of one command as input to another (e.g., Get-Process | Where CPU).
///   (, ): Subexpression operator; used to group expressions or invoke commands (e.g., (Get-Date).Year).
///   <, >: Input and output redirection.
///   $: Begins variable names (e.g., $name) and subexpressions (e.g., $($x + 1)).
///   .: Current directory reference (e.g., .\script.ps1) or method/property access (e.g., $obj.ToString()).
///   #: Begins a comment (only special at the start of a token, everything after is ignored by the parser).
///   @: When passed to external programs (like cl.exe), @filename may denote a response file (context-specific).
///   ': Used to create a literal string, meaning the content within the quotes is interpreted exactly as written, without variable expansion or command substitution.
///   <space>: Token separator; divides command, parameters, and arguments. Required between cmdlets, parameters, and values.
///
/// In the context of the flutter rust bridge Rust Powershell 5.1 caller use cases, only the \, ", and <space> metacharacters
/// have been identified (so far) as critically requiring escaping to allow strings such as:
///     --wasm-pack-rustflags=--cfg getrandom_backend=\"wasm_js\" -C target-feature=+atomics,+bulk-memory,+mutable-globals -C link-args=--shared-memory
/// to be escaped as single argument token as follows:
///     --wasm-pack-rustflags=--cfg` getrandom_backend=`\`"wasm_js`\`"` -C` target-feature=+atomics,+bulk-memory,+mutable-globals` -C` link-args=--shared-memory
/// This minimal set of escapes permits the execution of this command in the Windows Powershell 7 CLI terminal:
///     flutter_rust_bridge_codegen build-web "--wasm-pack-rustflags=--cfg getrandom_backend=`\`"wasm_js`\`" -C target-feature=+atomics,+bulk-memory,+mutable-globals -C link-args=--shared-memory"
pub fn windows_escape_for_powershell(section_in: &str) -> String {
    let mut token_out = String::new();
    for c in section_in.chars() {
        match c {
            '"' | '\\' | ' ' => token_out.push('`'),
            _ => (),
        }
        token_out.push(c);
    }
    token_out
}

#[derive(Default)]
pub(crate) struct ExecuteCommandOptions {
    pub envs: Option<HashMap<String, String>>,
    pub log_when_error: Option<bool>,
}

pub(crate) fn execute_command<'a>(
    bin: &str,
    args: impl IntoIterator<Item = &'a PathBuf>,
    current_dir: Option<&Path>,
    options: Option<ExecuteCommandOptions>,
) -> anyhow::Result<Output> {
    let options = options.unwrap_or_default();

    let args = args.into_iter().collect_vec();
    let args_display = args.iter().map(|path| path.to_string_lossy()).join(" ");
    let mut cmd = Command::new(bin);
    cmd.args(args);

    if let Some(current_dir) = current_dir {
        cmd.current_dir(normalize_windows_unc_path(&path_to_string(current_dir)?));
    }
    if let Some(envs) = options.envs {
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
    } else if options.log_when_error.unwrap_or(true) {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[cfg(windows)]
    fn test_call_shell_info() {
        let params = [
            "fvm",
            "dart",
            "run",
            "flutter_rust_bridge",
            "build-web",
            "--dart-root",
            "D:\\coding\\project",
            "--wasm-pack-rustflags=--cfg getrandom_backend=\\\"wasm_js\\\" -C target-feature=+atomics,+bulk-memory,+mutable-globals -C link-args=--shared-memory",
        ];
        let actual = call_shell_info(&params.into_iter().map(PathBuf::from).collect::<Vec<_>>());
        let cmd = "fvm dart run flutter_rust_bridge build-web --dart-root D:`\\coding`\\project --wasm-pack-rustflags=--cfg` getrandom_backend=`\\`\"wasm_js`\\`\"` -C` target-feature=+atomics,+bulk-memory,+mutable-globals` -C` link-args=--shared-memory";
        let expect = CommandInfo {
            program: "powershell".to_owned(),
            args: vec![
                "-noprofile".to_owned(),
                "-command".to_owned(),
                format!("& {}", cmd),
            ],
        };
        assert_eq!(actual, expect);
    }
    #[test]
    #[cfg(windows)]
    fn test_call_shell_info_escapes() {
        let params = ["abc\"def\\ghi jkl"];
        let actual = call_shell_info(&params.into_iter().map(PathBuf::from).collect::<Vec<_>>());
        let cmd = "abc`\"def`\\ghi` jkl";
        let expect = CommandInfo {
            program: "powershell".to_owned(),
            args: vec![
                "-noprofile".to_owned(),
                "-command".to_owned(),
                format!("& {}", cmd),
            ],
        };
        assert_eq!(actual, expect);
    }
    #[test]
    fn test_windows_escape_for_powershell() {
        let section_in =
            "detects regression \"errors\" when tests are run \\ on non_windows systems";
        let actual_token_out = windows_escape_for_powershell(&section_in);
        let expect_token_out = "detects` regression` `\"errors`\"` when` tests` are` run` `\\` on` non_windows` systems";
        assert_eq!(actual_token_out, expect_token_out);
    }
}
