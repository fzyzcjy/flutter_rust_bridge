use std::fmt::Write;
use std::path::Path;
use std::process::Command;
use std::process::Output;

use crate::error::{Error, Result};
use crate::utils::PackageManager;
use crate::utils::{ensure_dependencies, guess_toolchain, DartToolchain};
use log::{debug, info, warn};

#[must_use]
fn call_shell(cmd: &str) -> Output {
    #[cfg(windows)]
    return execute_command("powershell", &["-noprofile", "-c", cmd], None);

    #[cfg(not(windows))]
    execute_command("sh", &["-c", cmd], None)
}

pub fn ensure_tools_available(dart_root: &str) -> Result {
    let toolchain = guess_toolchain(dart_root).unwrap();
    match toolchain {
        DartToolchain::Dart if !call_shell("dart --version").status.success() => {
            return Err(Error::MissingExe("dart".to_string()));
        }
        DartToolchain::Flutter if !call_shell("flutter --version").status.success() => {
            return Err(Error::MissingExe("flutter".to_string()));
        }
        _ => {}
    };

    let ffi = ensure_dependencies(dart_root, "ffi", PackageManager::Dependencies).unwrap();
    if ffi.is_none() {
        return Err(Error::MissingDep {
            name: "ffi".into(),
            context: PackageManager::Dependencies,
            version: "^2.0.1".into(),
        });
    }

    let ffigen = ensure_dependencies(dart_root, "ffigen", PackageManager::DevDependencies).unwrap();
    if ffigen.is_none() {
        return Err(Error::MissingDep {
            name: "ffigen".into(),
            context: PackageManager::DevDependencies,
            version: "^6.0.1".into(),
        });
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
        export: cbindgen::ExportConfig {
            include: c_struct_names
                .iter()
                .map(|name| format!("\"{}\"", name))
                .collect::<Vec<_>>(),
            exclude: exclude_symbols,
            ..Default::default()
        },
        ..Default::default()
    };

    debug!("cbindgen config: {:?}", config);

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

    let cmd = if guess_toolchain(dart_root).unwrap() == DartToolchain::Dart {
        "dart run"
    } else {
        "flutter pub run"
    };
    let res = call_shell(&format!(
        "cd {}{}{} ffigen --config \"{}\"",
        dart_root,
        if cfg!(windows) { "; " } else { " && " },
        cmd,
        config_file.path().to_string_lossy()
    ));
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

pub fn format_rust(path: &str) -> Result {
    debug!("execute format_rust path={}", path);
    let res = execute_command("rustfmt", &[path], None);
    if !res.status.success() {
        return Err(Error::Rustfmt(
            String::from_utf8_lossy(&res.stderr).to_string(),
        ));
    }
    Ok(())
}

pub fn format_dart(path: &str, line_length: i32) -> Result {
    debug!(
        "execute format_dart path={} line_length={}",
        path, line_length
    );
    let res = call_shell(&format!(
        "dart format {} --line-length {}",
        path, line_length
    ));
    if !res.status.success() {
        return Err(Error::Dartfmt(
            String::from_utf8_lossy(&res.stderr).to_string(),
        ));
    }
    Ok(())
}

pub fn build_runner(dart_root: &str) -> Result {
    info!("Running build_runner at {}", dart_root);
    let toolchain = guess_toolchain(dart_root).unwrap();
    let out = if cfg!(windows) {
        call_shell(&format!(
            "cd \"{}\"; {} run build_runner build --delete-conflicting-outputs",
            dart_root,
            toolchain.as_run_command()
        ))
    } else {
        call_shell(&format!(
            "cd \"{}\" && {} run build_runner build --delete-conflicting-outputs",
            dart_root,
            toolchain.as_run_command()
        ))
    };
    if !out.status.success() {
        return Err(Error::StringError(format!(
            "Failed to run build_runner for {}: {}",
            dart_root,
            String::from_utf8_lossy(&out.stdout)
        )));
    }
    Ok(())
}
