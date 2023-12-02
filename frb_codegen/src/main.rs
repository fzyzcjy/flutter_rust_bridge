use anyhow::Context;
#[cfg(feature = "serde")]
use lib_flutter_rust_bridge_codegen::dump;
use lib_flutter_rust_bridge_codegen::{
    frb_codegen_multi, init_logger, parse_configs_and_symbols, RawOpts,
};
use log::{debug, error, info};

fn main() -> anyhow::Result<()> {
    //  get valiable options from user input command
    let raw_opts = RawOpts::try_parse_args_or_yaml()?;
    init_logger("./logs/", raw_opts.verbose)?;

    #[cfg(feature = "serde")]
    let dump_config = raw_opts.dump.clone();

    let (all_configs, all_symbols) = parse_configs_and_symbols(raw_opts)?;
    debug!("configs={:?}", all_configs);

    // dump config(s)
    #[cfg(feature = "serde")]
    if let Some(dump) = dump_config {
        return dump::dump_multi(&all_configs, dump).context("Failed to dump config");
    }

    // parse config(s)
    let mut errors = vec![];
    for config in all_configs.iter_all() {
        if let Err(err) = frb_codegen_multi(config, &all_configs, &all_symbols) {
            if config.keep_going {
                errors.push((&config.rust_input_path, err));
                continue;
            }
            error!("Fatal error encountered. Rerun with RUST_BACKTRACE=1 or RUST_BACKTRACE=full for more details.");
            return Err(err);
        }
    }
    if !errors.is_empty() {
        error!("Codegen failed with {} error(s).", errors.len());
        for (path, error) in &errors {
            error!("Error running codegen for {path}:\n{error}");
        }
        info!("Rerun with RUST_BACKTRACE=1 or RUST_BACKTRACE=full for more details.");
        std::process::exit(1)
    }

    info!("Now go and use it :)");
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use lazy_static::lazy_static;
    use lib_flutter_rust_bridge_codegen::{
        frb_codegen, frb_codegen_multi, init_logger, parse_configs_and_symbols, RawOpts,
    };
    use log::error;

    lazy_static! {
        static ref LOGGER: () = init_logger(".", true).unwrap();
    }

    // VS Code runs in frb_codegen with "Run test" and flutter_rust_bridge with "Debug test" >_>
    fn set_dir() {
        if let Ok(metadata) = fs::metadata("frb_codegen") {
            if metadata.is_dir() {
                std::env::set_current_dir("frb_codegen").unwrap();
            }
        }
    }

    fn run_cargo_test_command(test_case: &str) -> std::process::ExitStatus {
        let status = std::process::Command::new("cargo")
            .current_dir(format!("../frb_example/{test_case}/rust"))
            .arg("build")
            .spawn()
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    format!(
                        "`{test_case}`: Failed to execute 'cargo': program not found on {OS}",
                        OS = std::env::consts::OS
                    )
                } else {
                    format!(
                        "`{test_case}`: Failed to execute 'cargo': {error} on {OS}",
                        error = e,
                        OS = std::env::consts::OS
                    )
                }
            })
            .unwrap()
            .wait()
            .map_err(|e| {
                format!(
                    "`{test_case}`: Failed to wait for 'cargo': {error} on {OS}",
                    error = e,
                    OS = std::env::consts::OS
                )
            })
            .unwrap();
        status
    }

    fn run_dart_test_command(test_case: &str, absolute_path: PathBuf) -> std::process::ExitStatus {
        // 1.decide which dart command is valid in the specific system
        let mut dart = if cfg!(target_os = "windows") {
            "dart.bat"
        } else {
            "dart"
        };
        // check command validation
        if std::process::Command::new(dart)
            .arg("--version")
            .status()
            .is_err()
        {
            dart = if cfg!(target_os = "windows") {
                "dart"
            } else {
                "dart.bat"
            };

            // check command validation again
            if std::process::Command::new(dart)
                .arg("--version")
                .status()
                .is_err()
            {
                panic!("Failed to find 'dart' or 'dart.bat' command in the system path.");
            }
        }

        // 2. do the dart test check
        let status = std::process::Command::new(dart)
            .arg(format!("../frb_example/{test_case}/dart/lib/main.dart"))
            .arg(absolute_path)
            .spawn()
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    format!(
                        "`{test_case}`: Failed to execute '{dart}': program not found on {OS}",
                        OS = std::env::consts::OS
                    )
                } else {
                    format!(
                        "`{test_case}`: Failed to execute '{dart}': {error} on {OS}",
                        error = e,
                        OS = std::env::consts::OS
                    )
                }
            })
            .unwrap()
            .wait()
            .map_err(|e| {
                format!(
                    "`{test_case}`: Failed to wait for '{dart}': {error} on {OS}",
                    error = e,
                    OS = std::env::consts::OS
                )
            })
            .unwrap();
        status
    }

    /// When the `frb_example/pure_dart` fails to build, i.e. the `cargo build` there fails,
    /// i.e. the frb_codegen fails to generate code in that repo, you may want to use this
    /// test to examine the problems, because this one is a copy of `build.rs` in that
    /// `frb_example/pure_dart`. For example, you may run this `fn pure_dart()` unit
    /// test in the debugger.
    ///
    /// In some scenarios, such as when using VSCode to execute this test, the `cargo build`
    /// will be run before this `fn pure_dart()` test gets executed (see #1106 for details).
    /// Therefore, you may even fail to execute *this* function. In that case, you may run:
    /// `mv ../frb_example/pure_dart/rust/build.rs ../frb_example/pure_dart/rust/_build.rs`
    /// Then that `build.rs` is temporarily disabled and cargo build can run.
    #[test]
    fn pure_dart() {
        let test_case = "pure_dart";

        assert!(cfg!(feature = "chrono"));
        assert!(cfg!(feature = "uuid"));

        set_dir();

        *LOGGER;

        // Options for frb_codegen
        let raw_opts = RawOpts {
            config_file: Some("../frb_example/pure_dart/rust/.flutter_rust_bridge.yml".into()),
            ..Default::default()
        };

        // get opts from raw opts
        let (all_configs, all_symbols) = parse_configs_and_symbols(raw_opts).unwrap();

        // generation of rust api for ffi (single block)
        assert!(!all_configs.is_multi_blocks_case());
        frb_codegen(&all_configs[0], &all_symbols).unwrap();

        let _status = run_cargo_test_command(test_case);

        let output_path = PathBuf::from(
            #[cfg(target_os = "macos")]
            "../target/debug/libflutter_rust_bridge_example_pure_dart.dylib",
            #[cfg(target_os = "linux")]
            "../target/debug/libflutter_rust_bridge_example_pure_dart.so",
            #[cfg(target_os = "windows")]
            "../target/debug/flutter_rust_bridge_example_pure_dart.dll",
        );
        let absolute_path = fs::canonicalize(output_path).expect("Failed to get absolute path");
        println!("Absolute path to output: {:?}", absolute_path);

        if absolute_path.exists() {
            println!("Output file exists");
        } else {
            println!("Output file does not exist");
        }

        let status = run_dart_test_command(test_case, absolute_path);
        assert!(status.success());
    }

    /// See the documentation for the `pure_dart` test
    #[test]
    fn pure_dart_multi() {
        let test_case = "pure_dart_multi";

        set_dir();
        *LOGGER;

        // Options for frb_codegen
        let raw_opts = RawOpts {
            config_file: Some(
                "../frb_example/pure_dart_multi/rust/.flutter_rust_bridge.yml".into(),
            ),
            ..Default::default()
        };

        // get opts from raw opts
        let (all_configs, all_symbols) = parse_configs_and_symbols(raw_opts).unwrap();

        // parse config(s)
        let mut errors = vec![];
        for config in all_configs.iter_all() {
            if let Err(err) = frb_codegen_multi(config, &all_configs, &all_symbols) {
                if config.keep_going {
                    errors.push((&config.rust_input_path, err));
                    continue;
                }
                error!("Fatal error encountered. Rerun with RUST_BACKTRACE=1 or RUST_BACKTRACE=full for more details.");
                panic!()
            }
        }

        let _status = run_cargo_test_command(test_case);

        let output_path = PathBuf::from(
            #[cfg(target_os = "macos")]
            "../target/debug/libflutter_rust_bridge_example_pure_dart_multi.dylib",
            #[cfg(target_os = "linux")]
            "../target/debug/libflutter_rust_bridge_example_pure_dart_multi.so",
            #[cfg(target_os = "windows")]
            "../target/debug/flutter_rust_bridge_example_pure_dart_multi.dll",
        );
        let absolute_path = fs::canonicalize(output_path).expect("Failed to get absolute path");
        println!("Absolute path to output: {:?}", absolute_path);

        if absolute_path.exists() {
            println!("Output file exists");
        } else {
            println!("Output file does not exist");
        }

        let status = run_dart_test_command(test_case, absolute_path);
        assert!(status.success());
    }
}
