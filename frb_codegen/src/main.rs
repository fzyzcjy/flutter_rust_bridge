#[cfg(feature = "serde")]
use lib_flutter_rust_bridge_codegen::dump;
use lib_flutter_rust_bridge_codegen::{config_parse, frb_codegen_multi, init_logger, RawOpts};
use log::{debug, error, info};

fn main() -> anyhow::Result<()> {
    //  get valiable options from user input command
    let raw_opts = RawOpts::try_parse_args_or_yaml()?;
    init_logger("./logs/", raw_opts.verbose)?;

    #[cfg(feature = "serde")]
    let dump_config = raw_opts.dump.clone();

    let (all_configs, all_symbols) = config_parse(raw_opts)?;
    debug!("configs={:?}", all_configs);

    // generation of rust api for ffi
    #[cfg(feature = "serde")]
    if let Some(dump) = dump_config {
        return dump::dump_multi(&all_configs, dump);
    }

    for config_index in 0..all_configs.len() {
        if let Err(err) = frb_codegen_multi(&all_configs, config_index, &all_symbols) {
            error!("fatal: {err}");
            std::process::exit(1);
        }
    }

    info!("Now go and use it :)");
    Ok(())
}

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use lib_flutter_rust_bridge_codegen::{
        config_parse, frb_codegen, frb_codegen_multi, init_logger, RawOpts,
    };

    lazy_static! {
        static ref LOGGER: () = init_logger(".", true).unwrap();
    }

    // VS Code runs in frb_codegen with "Run test" and flutter_rust_bridge with "Debug test" >_>
    fn set_dir() {
        use std::fs;

        if let Ok(metadata) = fs::metadata("frb_codegen") {
            if metadata.is_dir() {
                std::env::set_current_dir("frb_codegen").unwrap();
            }
        }
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
        assert!(cfg!(feature = "chrono"));
        assert!(cfg!(feature = "uuid"));

        use std::process::Command;

        set_dir();

        // Options for frb_codegen
        let raw_opts = RawOpts {
            config_file: Some("../frb_example/pure_dart/rust/.flutter_rust_bridge.yml".into()),
            ..Default::default()
        };

        // get opts from raw opts
        let (all_configs, all_symbols) = config_parse(raw_opts).unwrap();

        // generation of rust api for ffi (single block)
        assert_eq!(all_configs.len(), 1);
        frb_codegen(&all_configs[0], &all_symbols).unwrap();

        let status = Command::new("cargo")
            .current_dir("../frb_example/pure_dart/rust")
            .arg("build")
            .spawn()
            .expect("failed to execute cargo")
            .wait()
            .expect("failed to wait for cargo to finish");
        assert!(status.success(), "cargo build failed");

        let status = Command::new("dart")
            .arg("../frb_example/pure_dart/dart/lib/main.dart")
            .arg("../target/debug/libflutter_rust_bridge_example.so")
            .spawn()
            .expect("failed to execute pure_dart")
            .wait()
            .expect("failed to wait for pure_dart");
        assert!(status.success(), "pure_dart failed");
    }

    /// See the documentation for the `pure_dart` test
    #[test]
    fn pure_dart_multi() {
        use std::process::Command;

        set_dir();

        /// Path of input Rust code
        const RUST_INPUT_1: &str = "../frb_example/pure_dart_multi/rust/src/api_1.rs";
        const RUST_INPUT_2: &str = "../frb_example/pure_dart_multi/rust/src/api_2.rs";
        /// Path of output generated Dart code
        const DART_OUTPUT_1: &str =
            "../frb_example/pure_dart_multi/dart/lib/bridge_generated_api_1.dart";
        const DART_OUTPUT_2: &str =
            "../frb_example/pure_dart_multi/dart/lib/bridge_generated_api_2.dart";
        /// Path of output Rust code
        const RUST_OUTPUT_1: &str = "../frb_example/pure_dart_multi/rust/src/generated_api_1.rs";
        const RUST_OUTPUT_2: &str = "../frb_example/pure_dart_multi/rust/src/generated_api_2.rs";
        /// Class name to use in dart, corresponding to each Rust block
        const CLASS_NAME_1: &str = "ApiClass1";
        const CLASS_NAME_2: &str = "ApiClass2";

        // Options for frb_codegen
        let mut raw_opts = RawOpts {
            // Path of input Rust code
            rust_input: vec![RUST_INPUT_1.to_string(), RUST_INPUT_2.to_string()],
            // Path of output generated Dart code
            dart_output: vec![DART_OUTPUT_1.to_string(), DART_OUTPUT_2.to_string()],
            // Path of output Rust code
            rust_output: Some(vec![RUST_OUTPUT_1.to_string(), RUST_OUTPUT_2.to_string()]),
            wasm: true,
            // Class name of each Rust block of api
            class_name: Some(vec![CLASS_NAME_1.to_string(), CLASS_NAME_2.to_string()]),
            dart_format_line_length: 120,
            // for other options use defaults
            ..Default::default()
        };

        if cfg!(feature = "c-output") {
            raw_opts.c_output = Some(vec![
                // each field should contain head file name
                "../frb_example/pure_dart_multi/rust/c_output_path/c_output_1.h".into(),
                "../frb_example/pure_dart_multi/rust/c_output_path/c_output_2.h".into(),
            ]);
        }

        if cfg!(feature = "extra-c-output-path") {
            raw_opts.extra_c_output_path = Some(vec![
                // For test, the below 2 paths format are made a little different
                "../frb_example/pure_dart_multi/rust/./extra_c_output_path_1/".into(),
                "../frb_example/pure_dart_multi/rust/extra_c_output_path_2".into(),
            ]);
        }

        // get opts from raw opts
        let (all_configs, all_symbols) = config_parse(raw_opts).unwrap();

        // generation of rust api for ffi (multi-blocks)
        for config_index in 0..all_configs.len() {
            frb_codegen_multi(&all_configs, config_index, &all_symbols).unwrap()
        }

        let status = Command::new("cargo")
            .current_dir("../frb_example/pure_dart_multi/rust")
            .arg("build")
            .spawn()
            .expect("failed to execute cargo")
            .wait()
            .expect("failed to wait for cargo to finish");
        assert!(status.success(), "cargo build failed");

        let status = Command::new("dart")
            .arg("../frb_example/pure_dart_multi/dart/lib/main.dart")
            .arg("../target/debug/libflutter_rust_bridge_example_multi.so")
            .spawn()
            .expect("failed to execute pure_dart")
            .wait()
            .expect("failed to wait for pure_dart");
        assert!(status.success(), "pure_dart failed");
    }
}
