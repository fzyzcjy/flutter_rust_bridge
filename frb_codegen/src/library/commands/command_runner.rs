use crate::utils::console::println_over_progress;
use crate::utils::path_utils::{normalize_windows_unc_path, path_to_string};
use anyhow::{bail, Context};
use itertools::Itertools;
use log::debug;
use log::warn;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::thread;

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
        CommandInfo {
            program: "powershell".to_owned(),
            args: vec![
                "-noprofile".to_owned(),
                "-command".to_owned(),
                format!("& {}", cmd),
            ],
        }
    }
    #[cfg(not(windows))]
    {
        let cmd = cmd.iter().map(|section| format!("{section:?}")).join(" ");
        CommandInfo {
            program: "sh".to_owned(),
            args: vec!["-c".to_owned(), cmd],
        }
    }
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
    pub stream_output: bool,
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

    let result = (if options.stream_output {
        execute_command_streaming(&mut cmd, bin, &args_display)
    } else {
        cmd.output().map_err(anyhow::Error::from)
    })
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
    } else if options.log_when_error.unwrap_or(true) && !options.stream_output {
        // Streaming already printed the child output live; do not dump it again.
        warn!(
            "command={:?} stdout={} stderr={}",
            cmd,
            stdout,
            String::from_utf8_lossy(&result.stderr)
        );
    }
    Ok(result)
}

fn execute_command_streaming(
    cmd: &mut Command,
    bin: &str,
    args_display: &str,
) -> anyhow::Result<Output> {
    cmd.stdin(Stdio::null());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    let mut child = cmd
        .spawn()
        .with_context(|| format!(r#""{bin}" "{args_display}" failed to spawn (cmd={cmd:?})"#))?;
    let stdout_pipe = child.stdout.take().context("stdout was piped")?;
    let stderr_pipe = child.stderr.take().context("stderr was piped")?;

    let stdout_thread = thread::spawn(move || {
        let mut collected = Vec::new();
        tee_reader(stdout_pipe, &mut collected);
        collected
    });
    let stderr_thread = thread::spawn(move || {
        let mut collected = Vec::new();
        tee_reader(stderr_pipe, &mut collected);
        collected
    });

    let status = child
        .wait()
        .with_context(|| format!(r#""{bin}" "{args_display}" failed to wait"#))?;
    // A panicked reader thread should not hide the child's exit status.
    let stdout = stdout_thread.join().unwrap_or_default();
    let stderr = stderr_thread.join().unwrap_or_default();
    Ok(Output {
        status,
        stdout,
        stderr,
    })
}

/// Copies `reader` into `collected` and prints each line above the codegen spinner.
fn tee_reader(reader: impl Read, collected: &mut Vec<u8>) {
    let mut reader = BufReader::new(reader);
    let mut line = Vec::new();
    loop {
        line.clear();
        match reader.read_until(b'\n', &mut line) {
            Ok(0) => break,
            Ok(_) => {
                collected.extend_from_slice(&line);
                let text = String::from_utf8_lossy(&line);
                println_over_progress(text.trim_end_matches(['\r', '\n']));
            }
            Err(_) => break,
        }
    }
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
    use std::io::Read;
    use std::time::{Duration, Instant};

    const STREAMING_STDIN_TEST_CHILD: &str = "FRB_STREAMING_STDIN_TEST_CHILD";
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
        let actual_token_out = windows_escape_for_powershell(section_in);
        let expect_token_out = "detects` regression` `\"errors`\"` when` tests` are` run` `\\` on` non_windows` systems";
        assert_eq!(actual_token_out, expect_token_out);
    }

    #[test]
    fn test_tee_reader_collects_every_line_including_crlf() {
        let input = b"freezed on 45 inputs\r\nE missing implementation\npartial";
        let mut collected = Vec::new();
        super::tee_reader(&input[..], &mut collected);
        assert_eq!(collected, input);
    }

    #[test]
    fn test_tee_reader_empty_input_is_noop() {
        let mut collected = Vec::new();
        super::tee_reader(&b""[..], &mut collected);
        assert!(collected.is_empty());
    }

    struct FailingReader;

    impl Read for FailingReader {
        fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "boom"))
        }
    }

    #[test]
    fn test_tee_reader_io_error_stops_without_panic() {
        let mut collected = Vec::new();
        super::tee_reader(FailingReader, &mut collected);
        assert!(collected.is_empty());
    }

    fn streaming_options() -> Option<ExecuteCommandOptions> {
        Some(ExecuteCommandOptions {
            stream_output: true,
            ..Default::default()
        })
    }

    fn cmd_args(parts: &[&str]) -> Vec<PathBuf> {
        parts.iter().map(PathBuf::from).collect()
    }

    #[test]
    fn test_execute_command_streaming_captures_stdout() {
        let (bin, args) = if cfg!(windows) {
            ("cmd", cmd_args(&["/c", "echo", "stream-ok"]))
        } else {
            ("echo", cmd_args(&["stream-ok"]))
        };
        let out = execute_command(bin, args.iter(), None, streaming_options()).unwrap();
        assert!(out.status.success());
        let stdout = String::from_utf8_lossy(&out.stdout);
        if !stdout.contains("stream-ok") {
            // Failure-only diagnostics; a passing test never takes this arm.
            // frb-coverage:ignore-start
            panic!("expected streamed stdout to contain marker, got {stdout:?}");
            // frb-coverage:ignore-end
        }
    }

    #[test]
    fn test_execute_command_streaming_captures_stderr_and_nonzero_exit() {
        let (bin, args) = if cfg!(windows) {
            ("cmd", cmd_args(&["/c", "echo stream-err 1>&2 & exit /b 1"]))
        } else {
            ("sh", cmd_args(&["-c", "echo stream-err >&2; exit 1"]))
        };
        let out = execute_command(bin, args.iter(), None, streaming_options()).unwrap();
        assert!(!out.status.success());
        let stderr = String::from_utf8_lossy(&out.stderr);
        if !stderr.contains("stream-err") {
            // Failure-only diagnostics; a passing test never takes this arm.
            // frb-coverage:ignore-start
            panic!("expected streamed stderr to contain marker, got {stderr:?}");
            // frb-coverage:ignore-end
        }
    }

    #[test]
    fn test_execute_command_streaming_missing_binary_fails() {
        let err = execute_command(
            "definitely-not-a-real-binary-frb-stream-test-9f3c",
            std::iter::empty(),
            None,
            streaming_options(),
        )
        .expect_err("missing binary should fail to spawn");
        let msg = format!("{err:#}");
        if !msg.contains("failed to spawn") {
            // Failure-only diagnostics; a passing test never takes this arm.
            // frb-coverage:ignore-start
            panic!("expected spawn error, got {msg}");
            // frb-coverage:ignore-end
        }
    }

    #[test]
    /// Streaming children must receive EOF instead of inheriting caller stdin.
    fn test_execute_command_streaming_closes_stdin() {
        if std::env::var_os(STREAMING_STDIN_TEST_CHILD).is_some() {
            let (bin, args) = if cfg!(windows) {
                ("cmd", cmd_args(&["/c", "set /p FRB_INPUT= & exit /b 0"]))
            } else {
                ("sh", cmd_args(&["-c", "read frb_input || true"]))
            };
            let output = execute_command(bin, args.iter(), None, streaming_options()).unwrap();
            assert!(output.status.success());
            std::process::exit(42);
        }

        let mut test_process = Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "library::commands::command_runner::tests::test_execute_command_streaming_closes_stdin",
                "--nocapture",
            ])
            .env(STREAMING_STDIN_TEST_CHILD, "1")
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let deadline = Instant::now() + Duration::from_secs(3);

        loop {
            if let Some(status) = test_process.try_wait().unwrap() {
                assert_eq!(status.code(), Some(42));
                break;
            }
            if Instant::now() >= deadline {
                drop(test_process.stdin.take());
                let _ = test_process.wait();
                panic!("streaming child inherited an open stdin pipe");
            }
            thread::sleep(Duration::from_millis(10));
        }
    }
}
