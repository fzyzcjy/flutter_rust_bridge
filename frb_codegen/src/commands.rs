use std::fmt::Write;
use std::path::Path;
use std::process::Command;
use std::process::Output;

use log::{debug, error, info, warn};

/// Known failures that occur from external commands.
/// If an error occurs frequently enough, consider adding it here and use
/// [std::process::exit] explicitly instead of panicking.
enum Failures {
    Rustfmt = 1,
    Dartfmt,
    FfigenLlvm,
    MissingExe,
}

#[must_use]
fn call_shell(cmd: &str) -> Output {
    #[cfg(windows)]
    return execute_command("powershell", &["-noprofile", "-c", cmd], None);

    #[cfg(not(windows))]
    execute_command("sh", &["-c", cmd], None)
}

pub fn ensure_tools_available() {
    let output = call_shell("dart pub global list");
    let output = String::from_utf8_lossy(&output.stdout);
    if !output.contains("ffigen") {
        error!(
            "
ffigen is not available, please run \"dart pub global activate ffigen\" first."
        );
        std::process::exit(Failures::MissingExe as _);
    }

    check_shell_executable("cbindgen");
}

pub fn check_shell_executable(cmd: &'static str) {
    #[cfg(windows)]
    let res = execute_command("where", &[cmd], None);
    #[cfg(not(windows))]
    let res = execute_command(
        "sh",
        &["-c", &format!("test -x \"$(which {})\"", cmd)],
        None,
    );
    if !res.status.success() {
        error!(
            "
{cmd} is not a command, or not executable.
Note: This command might be available via cargo, in which case it can be installed with:

    cargo install {cmd}",
            cmd = cmd
        );
        std::process::exit(Failures::MissingExe as _);
    }
    debug!("{}", String::from_utf8_lossy(&res.stdout));
}

pub fn bindgen_rust_to_dart(
    rust_crate_dir: &str,
    c_output_path: &str,
    dart_output_path: &str,
    dart_class_name: &str,
    c_struct_names: Vec<String>,
    llvm_install_path: &[String],
    llvm_compiler_opts: &str,
) {
    cbindgen(rust_crate_dir, c_output_path, c_struct_names);
    ffigen(
        c_output_path,
        dart_output_path,
        dart_class_name,
        llvm_install_path,
        llvm_compiler_opts,
    );
}

#[must_use = "Error path must be handled."]
fn execute_command(bin: &str, args: &[&str], current_dir: Option<&str>) -> Output {
    let mut cmd = Command::new(bin);
    cmd.args(args);

    if let Some(current_dir) = current_dir {
        cmd.current_dir(current_dir);
    }

    debug!(
        "execute command: bin={} args={:?} current_dir={:?} cmd={:?}",
        bin, args, current_dir, cmd
    );

    let result = cmd
        .output()
        .unwrap_or_else(|err| panic!("\"{}\" \"{}\" failed: {}", bin, args.join(" "), err));

    let stdout = String::from_utf8_lossy(&result.stdout);
    if result.status.success() {
        debug!(
            "command={:?} stdout={} stderr={}",
            cmd,
            stdout,
            String::from_utf8_lossy(&result.stderr)
        );
        if stdout.contains("fatal error") {
            warn!("See keywords such as `error` in command output. Maybe there is a problem? command={:?} output={:?}", cmd, result);
        } else if args.contains(&"ffigen") && stdout.contains("[SEVERE]") {
            // HACK: If ffigen can't find a header file it will generate broken
            // bindings but still exit successfully. We can detect these broken
            // bindings by looking for a "[SEVERE]" log message.
            //
            // It may emit SEVERE log messages for non-fatal errors though, so
            // we don't want to error out completely.

            warn!(
                "The `ffigen` command emitted a SEVERE error. Maybe there is a problem? command={:?} output=\n{}",
                cmd, String::from_utf8_lossy(&result.stdout)
            );
        }
    } else {
        warn!(
            "command={:?} stdout={} stderr={}",
            cmd,
            stdout,
            String::from_utf8_lossy(&result.stderr)
        );
    }
    result
}

fn cbindgen(rust_crate_dir: &str, c_output_path: &str, c_struct_names: Vec<String>) {
    debug!(
        "execute cbindgen rust_crate_dir={} c_output_path={}",
        rust_crate_dir, c_output_path
    );

    let config = format!(
        r#"
language = "C"

# do NOT include "stdarg.h", see #108 and #53
sys_includes = ["stdbool.h", "stdint.h", "stdlib.h"]
no_includes = true

[export]
include = [{}]
"#,
        c_struct_names
            .iter()
            .map(|name| format!("\"{}\"", name))
            .collect::<Vec<_>>()
            .join(", ")
    );
    debug!("cbindgen config: {}", config);

    let mut config_file = tempfile::NamedTempFile::new().unwrap();
    std::io::Write::write_all(&mut config_file, config.as_bytes()).unwrap();
    debug!("cbindgen config_file: {:?}", config_file);

    let canonical = Path::new(rust_crate_dir)
        .canonicalize()
        .expect("Could not canonicalize rust crate dir");
    let mut path = canonical.to_str().unwrap();

    // on windows get rid of the UNC path
    if path.starts_with(r"\\?\") {
        path = &path[r"\\?\".len()..];
    }

    let res = execute_command(
        "cbindgen",
        &[
            "-v",
            "--config",
            config_file.path().to_str().unwrap(),
            "--output",
            c_output_path,
        ],
        Some(path),
    );
    if !res.status.success() {
        panic!("cbindgen failed: {}", String::from_utf8_lossy(&res.stderr));
    }
}

fn ffigen(
    c_path: &str,
    dart_path: &str,
    dart_class_name: &str,
    llvm_path: &[String],
    llvm_compiler_opts: &str,
) {
    debug!(
        "execute ffigen c_path={} dart_path={} llvm_path={:?}",
        c_path, dart_path, llvm_path
    );
    let mut config = format!(
        "
        output: '{}'
        name: '{}'
        description: 'generated by flutter_rust_bridge'
        headers:
          entry-points:
            - '{}'
          include-directives:
            - '{}'
        comments: false
        preamble: |
          // ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names
        ",
        dart_path, dart_class_name, c_path, c_path,
    );
    if !llvm_path.is_empty() {
        write!(
            &mut config,
            "
        llvm-path:\n"
        )
        .unwrap();
        for path in llvm_path {
            writeln!(&mut config, "           - '{}'", path).unwrap();
        }
    }

    if !llvm_compiler_opts.is_empty() {
        config = format!(
            "{}
        compiler-opts:
            - '{}'",
            config, llvm_compiler_opts
        );
    }

    debug!("ffigen config: {}", config);

    let mut config_file = tempfile::NamedTempFile::new().unwrap();
    std::io::Write::write_all(&mut config_file, config.as_bytes()).unwrap();
    debug!("ffigen config_file: {:?}", config_file);

    // NOTE please install ffigen globally first: `dart pub global activate ffigen`
    let res = call_shell(&format!(
        "dart pub global run ffigen --config \"{}\"",
        config_file.path().to_string_lossy()
    ));
    if !res.status.success() {
        let err = String::from_utf8_lossy(&res.stderr);
        if err.contains("Couldn't find dynamic library in default locations.") {
            error!(
                "
ffigen could not find LLVM.
Please supply --llvm-path to flutter_rust_bridge_codegen, e.g.:

    flutter_rust_bridge_codegen .. --llvm-path <path_to_llvm>"
            );
            std::process::exit(Failures::FfigenLlvm as _);
        }
        panic!("ffigen failed:\n{}", err);
    }
}

pub fn format_rust(path: &str) {
    debug!("execute format_rust path={}", path);
    let res = execute_command("rustfmt", &[path], None);
    if !res.status.success() {
        error!("rustfmt failed: {}", String::from_utf8_lossy(&res.stderr));
        std::process::exit(Failures::Rustfmt as _);
    }
}

pub fn format_dart(path: &str, line_length: i32) {
    debug!(
        "execute format_dart path={} line_length={}",
        path, line_length
    );
    let res = call_shell(&format!(
        "dart format {} --line-length {}",
        path, line_length
    ));
    if !res.status.success() {
        error!(
            "dart format failed: {}",
            String::from_utf8_lossy(&res.stderr)
        );
        std::process::exit(Failures::Dartfmt as _);
    }
}

pub fn build_runner(dart_root: &str) {
    info!("Running build_runner at {}", dart_root);
    let out = if cfg!(windows) {
        call_shell(&format!(
            "cd \"{}\"; dart run build_runner build --delete-conflicting-outputs",
            dart_root
        ))
    } else {
        call_shell(&format!(
            "cd \"{}\" && dart run build_runner build --delete-conflicting-outputs",
            dart_root
        ))
    };
    if !out.status.success() {
        error!(
            "Failed to run build_runner for {}: {}",
            dart_root,
            String::from_utf8_lossy(&out.stdout)
        );
    }
}
