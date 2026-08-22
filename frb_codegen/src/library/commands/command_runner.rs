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
    let CommandInfo { program, args } = call_shell_info(cmd)?;
    let program = &program;
    command_run!(program in pwd, options = options, *args)
}

#[derive(Debug, PartialEq)]
pub(crate) struct CommandInfo {
    pub program: String,
    pub args: Vec<String>,
}

pub(crate) fn call_shell_info(cmd: &[PathBuf]) -> anyhow::Result<CommandInfo> {
    #[cfg(windows)]
    {
        let cmd = cmd
            .iter()
            .map(|section| windows_escape_for_powershell(section.to_str().unwrap()))
            .join(" ");
        Ok(CommandInfo {
            program: "powershell".to_owned(),
            args: vec![
                "-noprofile".to_owned(),
                "-command".to_owned(),
                format!("& {}", cmd),
            ],
        })
    }
    #[cfg(not(windows))]
    {
        let cmd = cmd
            .iter()
            .map(|section| shell_quote(section))
            .collect::<anyhow::Result<Vec<_>>>()?
            .join(" ");
        Ok(CommandInfo {
            program: "sh".to_owned(),
            args: vec!["-c".to_owned(), cmd],
        })
    }
}

#[cfg(not(windows))]
fn shell_quote(section: &Path) -> anyhow::Result<String> {
    Ok(format!(
        "'{}'",
        section
            .to_str()
            .context("shell argument is not valid UTF-8")?
            .replace('\'', "'\"'\"'")
    ))
}

/// Applies a minimal set of backtick escapes to convert a string into a PowerShell 5.1 argument token.
///
/// Note: The escapes are targeted for PowerShell 5.1 or earlier (`powershell.exe`) which is invoked
/// the by the calling call_shell_info() function, not PowerShell 7+ (`pwsh.exe`).
///
/// This function handles the escaping of metacharacters to ensure the input string will be correctly parsed
/// later by PowerShell 5.1 as a single, intact argument token. The following is a non-exhaustive list
/// PowerShell 5.1 argument-mode metacharacters to consider:
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
/// In the context of the flutter rust bridge Rust Powershell 5.1 caller use cases, only the \, " and <space> metacharacters
/// from the above list have been identified (so far) as critically requiring escaping to allow strings such as:
///     --wasm-pack-rustflags=--cfg getrandom_backend=\"wasm_js\" -C target-feature=+atomics,+bulk-memory,+mutable-globals -C link-args=--shared-memory
/// to be escaped and converted to a single argument token as follows:
///     --wasm-pack-rustflags=--cfg` getrandom_backend=`\`"wasm_js`\`"` -C` target-feature=+atomics,+bulk-memory,+mutable-globals` -C` link-args=--shared-memory
/// This minimal set of escapes permits the execution of this command in the Windows Powershell 7 CLI terminal:
///     PS> flutter_rust_bridge_codegen build-web "--wasm-pack-rustflags=--cfg getrandom_backend=`\`"wasm_js`\`" -C target-feature=+atomics,+bulk-memory,+mutable-globals -C link-args=--shared-memory"
/// This minimal set of escapes may need to be augmented in the future as windows users find other CLI use cases
/// that require additional escaping. If the windows users of the flutter rust bridge cannot agree on a single
/// minimal set of escapes, then the alternative is to supply an new argument option to the `flutter_rust_bridge_codegen`
/// CLI command that specifies the characters to be escaped for the argument tokens of the internal PowerShell 5.1 call, e.g.:
///     PS> flutter_rust_bridge_codegen build-web --ps51-escapes '"\ ' ...
#[cfg(any(windows, test))]
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

    /// Builds a POSIX shell command string for ordinary quoted arguments.
    #[test]
    #[cfg(not(windows))]
    fn builds_posix_shell_arguments_with_literal_quoting() {
        let actual = call_shell_info(&[
            PathBuf::from("tool"),
            PathBuf::from("argument with spaces"),
            PathBuf::from("quote\"and\\slash"),
        ])
        .unwrap();

        assert_eq!(
            actual,
            CommandInfo {
                program: "sh".to_owned(),
                args: vec![
                    "-c".to_owned(),
                    "'tool' 'argument with spaces' 'quote\"and\\slash'".to_owned(),
                ],
            }
        );
    }

    /// Preserves shell metacharacters and newlines as literal executable arguments.
    #[cfg(unix)]
    #[test]
    fn preserves_shell_metacharacters_as_literal_arguments() -> anyhow::Result<()> {
        use std::fs;
        use std::os::unix::ffi::OsStringExt;
        use std::os::unix::fs::PermissionsExt;
        use tempfile::tempdir;

        let directory = tempdir()?;
        let capture_path = directory.path().join("capture.sh");
        let output_path = directory.path().join("arguments.bin");
        fs::write(
            &capture_path,
            "#!/bin/sh\nprintf '%s\\0' \"$@\" > \"$CAPTURE_OUTPUT\"\n",
        )?;
        fs::set_permissions(&capture_path, fs::Permissions::from_mode(0o755))?;

        let expected = vec![
            PathBuf::from("dollar:$HOME"),
            PathBuf::from("substitution:$(printf command)"),
            PathBuf::from("backtick:`printf tick`"),
            PathBuf::from("quote:\"literal\""),
            PathBuf::from("apostrophe:'literal'"),
            PathBuf::from("newline:first\nsecond"),
        ];
        let options = ExecuteCommandOptions {
            envs: Some(HashMap::from([(
                "CAPTURE_OUTPUT".to_owned(),
                output_path.to_string_lossy().into_owned(),
            )])),
            ..Default::default()
        };
        let command = std::iter::once(capture_path)
            .chain(expected.iter().cloned())
            .collect_vec();

        check_exit_code(&call_shell(&command, None, Some(options))?)?;

        let actual = fs::read(output_path)?
            .split(|byte| *byte == 0)
            .filter(|argument| !argument.is_empty())
            .map(|argument| PathBuf::from(std::ffi::OsString::from_vec(argument.to_vec())))
            .collect_vec();
        assert_eq!(actual, expected);
        Ok(())
    }

    /// Rejects non-UTF-8 arguments instead of silently changing their bytes.
    #[cfg(unix)]
    #[test]
    fn rejects_non_utf8_shell_arguments() {
        use std::os::unix::ffi::OsStringExt;

        let argument = PathBuf::from(std::ffi::OsString::from_vec(vec![0xff]));

        assert!(call_shell_info(&[argument]).is_err());
    }

    /// Accepts an output with a successful exit status.
    #[test]
    fn accepts_successful_exit_status() {
        assert!(check_exit_code(&Output {
            status: success_exit_status(),
            stdout: Vec::new(),
            stderr: Vec::new(),
        })
        .is_ok());
    }

    /// Returns stderr context for an unsuccessful exit status.
    #[test]
    fn rejects_unsuccessful_exit_status() {
        let error = check_exit_code(&Output {
            status: failure_exit_status(),
            stdout: Vec::new(),
            stderr: b"missing tool".to_vec(),
        })
        .unwrap_err();

        assert!(error
            .to_string()
            .contains("Command execution failed: missing tool"));
    }

    #[cfg(unix)]
    fn success_exit_status() -> std::process::ExitStatus {
        use std::os::unix::process::ExitStatusExt;

        std::process::ExitStatus::from_raw(0)
    }

    #[cfg(windows)]
    fn success_exit_status() -> std::process::ExitStatus {
        use std::os::windows::process::ExitStatusExt;

        std::process::ExitStatus::from_raw(0)
    }

    #[cfg(unix)]
    fn failure_exit_status() -> std::process::ExitStatus {
        use std::os::unix::process::ExitStatusExt;

        std::process::ExitStatus::from_raw(1 << 8)
    }

    #[cfg(windows)]
    fn failure_exit_status() -> std::process::ExitStatus {
        use std::os::windows::process::ExitStatusExt;

        std::process::ExitStatus::from_raw(1)
    }

    #[test]
    #[cfg(windows)]
    /// Builds the PowerShell invocation for the complete build-web argument list.
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
        let actual =
            call_shell_info(&params.into_iter().map(PathBuf::from).collect::<Vec<_>>()).unwrap();
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
    /// Escapes PowerShell argument metacharacters in a single token.
    fn test_call_shell_info_escapes() {
        let params = ["abc\"def\\ghi jkl"];
        let actual =
            call_shell_info(&params.into_iter().map(PathBuf::from).collect::<Vec<_>>()).unwrap();
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
    /// Escapes spaces, quotes, and backslashes for PowerShell tokens.
    #[test]
    fn test_windows_escape_for_powershell() {
        let section_in =
            "detects regression \"errors\" when tests are run \\ on non_windows systems";
        let actual_token_out = windows_escape_for_powershell(section_in);
        let expect_token_out = "detects` regression` `\"errors`\"` when` tests` are` run` `\\` on` non_windows` systems";
        assert_eq!(actual_token_out, expect_token_out);
    }
}
