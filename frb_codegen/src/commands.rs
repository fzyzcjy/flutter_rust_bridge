use anyhow::anyhow;
use itertools::Itertools;
use log::log_enabled;
use std::fmt::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;
use std::process::Stdio;
use std::str::FromStr;

use crate::error::{Error, Result};
use crate::tools::DartRepository;
use crate::tools::PackageManager;
use crate::tools::FFIGEN_REQUIREMENT;
use crate::tools::FFI_REQUIREMENT;
use log::{debug, info, warn};

/// - First argument is either a string of a command, or a function receiving a slice of [`PathBuf`].
///   - The command may be followed by `in <expr>` to specify the working directory.
///   - The function may be followed by an array of rest parameters to pass.
/// - Following arguments are either:
///   - An expression to turn into a [`PathBuf`]; or
///   - `?<expr>` to add `expr` only if `expr` is a [`Some`]; or
///   - `*<expr>` to concatenate an iterable of such expressions; or
///   - A tuple of `(condition, expr, ...expr)` that adds `expr`s to the arguments only if `condition` is satisfied.
///
/// Returns [`Result<Output>`] if executing a command name, or the return value of the specified function.
#[doc(hidden)]
#[macro_export]
macro_rules! run {
    ($binary:literal, $($rest:tt)*) => {{
        let args = $crate::args!($($rest)*);
        $crate::commands::execute_command($binary, args.iter(), None)
    }};
    ($binary:literal in $pwd:expr, $($rest:tt)*) => {{
        let args = $crate::args!($($rest)*);
        $crate::commands::execute_command($binary, args.iter(), $pwd)
    }};
    ($command:path $([ $($args:expr),* ])?, $($rest:tt)*) => {{
        let args = $crate::args!($($rest)*);
        $command(&args[..] $(, $($args),* )?)
    }};
}

/// Formats a list of [`PathBuf`]s using the syntax detailed in [`run`].
#[doc(hidden)]
#[macro_export]
macro_rules! args {
    (@args $args:ident $(,)?) => {};
    (@args $args:ident ($cond:expr, $($expr:expr),+ $(,)?), $($rest:tt)*) => {
        if $cond {
            $(
                $args.push(::std::path::PathBuf::from($expr));
            )+
        }
        $crate::args!(@args $args $($rest)*);
    };
    (@args $args:ident ?$src:expr, $($rest:tt)*) => {
        if let Some(it) = (&$src) {
            $args.push(::std::path::PathBuf::from(it));
        }
        $crate::args!(@args $args $($rest)*);
    };
    (@args $args:ident *$src:expr, $($rest:tt)*) => {
        $args.extend($src.iter().map(::std::path::PathBuf::from));
        $crate::args!(@args $args $($rest)*);
    };
    (@args $args:ident $expr:expr, $($rest:tt)*) => {
        $args.push(::std::path::PathBuf::from($expr));
        $crate::args!(@args $args $($rest)*);
    };
    ($($rest:tt)*) => {{
        let mut args = Vec::new();
        $crate::args!(@args args $($rest)*,);
        args
    }};
}

pub(crate) fn call_shell(cmd: &[PathBuf], pwd: Option<&str>) -> Result<Output> {
    let cmd = cmd.iter().map(|section| format!("{:?}", section)).join(" ");
    #[cfg(windows)]
    {
        run!("powershell" in pwd, "-noprofile", "-command", format!("& {}", cmd))
    }

    #[cfg(not(windows))]
    run!("sh" in pwd, "-c", cmd)
}

pub fn ensure_tools_available(dart_root: &str, skip_deps_check: bool) -> Result {
    let repo =
        DartRepository::from_str(dart_root).map_err(|e| Error::StringError(e.to_string()))?;
    if !repo.toolchain_available() {
        return Err(Error::MissingExe(repo.toolchain.to_string()));
    }

    if !skip_deps_check {
        repo.has_specified("ffi", PackageManager::Dependencies, &FFI_REQUIREMENT)?;
        repo.has_installed("ffi", PackageManager::Dependencies, &FFI_REQUIREMENT)?;

        repo.has_specified(
            "ffigen",
            PackageManager::DevDependencies,
            &FFIGEN_REQUIREMENT,
        )?;
        repo.has_installed(
            "ffigen",
            PackageManager::DevDependencies,
            &FFIGEN_REQUIREMENT,
        )?;
    }

    Ok(())
}

pub(crate) struct BindgenRustToDartArg<'a> {
    pub rust_crate_dir: &'a str,
    pub c_output_path: &'a str,
    pub dart_output_path: &'a str,
    pub dart_class_name: &'a str,
    pub c_struct_names: Vec<String>,
    pub exclude_symbols: Vec<String>,
    pub llvm_install_path: &'a [String],
    pub llvm_compiler_opts: &'a str,
}

pub(crate) fn bindgen_rust_to_dart(
    arg: BindgenRustToDartArg,
    dart_root: &str,
) -> anyhow::Result<()> {
    cbindgen(
        arg.rust_crate_dir,
        arg.c_output_path,
        arg.c_struct_names,
        arg.exclude_symbols,
    )?;
    ffigen(
        arg.c_output_path,
        arg.dart_output_path,
        arg.dart_class_name,
        arg.llvm_install_path,
        arg.llvm_compiler_opts,
        dart_root,
    )
}

#[must_use = "Error path must be handled."]
fn execute_command<'a>(
    bin: &str,
    args: impl IntoIterator<Item = &'a std::path::PathBuf>,
    current_dir: Option<&str>,
) -> Result<Output> {
    let args = args.into_iter().collect::<Vec<_>>();
    let args_display = args.iter().map(|path| path.to_string_lossy()).join(" ");
    let mut cmd = Command::new(bin);
    cmd.args(args);
    if log_enabled!(log::Level::Info) {
        cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    }

    if let Some(current_dir) = current_dir {
        cmd.current_dir(current_dir);
    }
    if log_enabled!(log::Level::Info) {
        cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    }

    debug!(
        "execute command: bin={} args={:?} current_dir={:?} cmd={:?}",
        bin, args_display, current_dir, cmd
    );

    let result = cmd
        .output()
        .map_err(|err| anyhow!("\"{}\" \"{}\" failed: {}", bin, args_display, err))?;

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
        } else if args_display.contains("ffigen") && stdout.contains("[SEVERE]") {
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
    Ok(result)
}

fn cbindgen(
    rust_crate_dir: &str,
    c_output_path: &str,
    c_struct_names: Vec<String>,
    exclude_symbols: Vec<String>,
) -> anyhow::Result<()> {
    debug!(
        "execute cbindgen rust_crate_dir={} c_output_path={}",
        rust_crate_dir, c_output_path
    );
    let config = cbindgen::Config {
        language: cbindgen::Language::C,
        sys_includes: vec![
            "stdbool.h".to_string(),
            "stdint.h".to_string(),
            "stdlib.h".to_string(),
        ],
        no_includes: true,
        // copied from: dart-sdk/dart_api.h
        // used to convert Dart_Handle to Object.
        after_includes: Some("typedef struct _Dart_Handle* Dart_Handle;".to_owned()),
        export: cbindgen::ExportConfig {
            include: c_struct_names
                .iter()
                .map(|name| format!("\"{}\"", name))
                .collect(),
            exclude: exclude_symbols,
            ..Default::default()
        },
        ..Default::default()
    };

    debug!("cbindgen config: {:#?}", config);

    let canonical = Path::new(rust_crate_dir)
        .canonicalize()
        .expect("Could not canonicalize rust crate dir");
    let mut path = canonical.to_str().unwrap();

    // on windows get rid of the UNC path
    if path.starts_with(r"\\?\") {
        path = &path[r"\\?\".len()..];
    }

    if cbindgen::generate_with_config(path, config)?.write_to_file(c_output_path) {
        Ok(())
    } else {
        Err(Error::str("cbindgen failed writing file").into())
    }
}

fn ffigen(
    c_path: &str,
    dart_path: &str,
    dart_class_name: &str,
    llvm_path: &[String],
    llvm_compiler_opts: &str,
    dart_root: &str,
) -> anyhow::Result<()> {
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
        )?;
        for path in llvm_path {
            writeln!(&mut config, "           - '{}'", path)?;
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

    let mut config_file = tempfile::NamedTempFile::new()?;
    std::io::Write::write_all(&mut config_file, config.as_bytes())?;
    debug!("ffigen config_file: {:?}", config_file);

    let repo = DartRepository::from_str(dart_root).unwrap();
    let res = run!(
        call_shell[Some(dart_root)],
        *repo.toolchain.as_run_command(),
        "run",
        "ffigen",
        "--config",
        config_file.path()
    )?;
    if !res.status.success() {
        let err = String::from_utf8_lossy(&res.stderr);
        let out = String::from_utf8_lossy(&res.stdout);
        let pat = "Couldn't find dynamic library in default locations.";
        if err.contains(pat) || out.contains(pat) {
            return Err(Error::FfigenLlvm.into());
        }
        return Err(
            Error::string(format!("ffigen failed:\nstderr: {}\nstdout: {}", err, out)).into(),
        );
    }
    Ok(())
}

pub fn format_rust(path: &[PathBuf]) -> Result {
    debug!("execute format_rust path={:?}", path);
    let res = execute_command("rustfmt", path, None)?;
    if !res.status.success() {
        return Err(Error::Rustfmt(
            String::from_utf8_lossy(&res.stderr).to_string(),
        ));
    }
    Ok(())
}

pub fn format_dart(path: &[PathBuf], line_length: u32) -> Result {
    debug!(
        "execute format_dart path={:?} line_length={}",
        path, line_length
    );
    let res = run!(
        call_shell[None],
        "dart",
        "format",
        "--line-length",
        line_length.to_string(),
        *path
    )
    .map_err(|err| Error::StringError(format!("{}", err)))?;
    if !res.status.success() {
        return Err(Error::Dartfmt(
            String::from_utf8_lossy(&res.stderr).to_string(),
        ));
    }
    Ok(())
}

pub fn build_runner(dart_root: &str) -> Result {
    info!("Running build_runner at {}", dart_root);
    let repo = DartRepository::from_str(dart_root).unwrap();
    let out = run!(
        call_shell[Some(dart_root)],
        *repo.toolchain.as_run_command(),
        "run",
        "build_runner",
        "build",
        "--delete-conflicting-outputs"
    )?;
    if !out.status.success() {
        return Err(Error::StringError(format!(
            "Failed to run build_runner for {}: {}",
            dart_root,
            String::from_utf8_lossy(&out.stdout)
        )));
    }
    Ok(())
}
