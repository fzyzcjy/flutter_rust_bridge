use std::process::exit;

use clap::Parser;
use lib_flutter_rust_bridge_codegen::{
    config_parse, frb_codegen_multi, get_symbols_if_no_duplicates, init_logger, RawOpts,
};
use log::{debug, error, info};

fn main() -> anyhow::Result<()> {
    //  get valiable options from user input command
    let raw_opts = RawOpts::parse();
    init_logger("./logs/", raw_opts.verbose).unwrap();

    let configs = config_parse(raw_opts);
    debug!("configs={:?}", configs);

    // generation of rust api for ffi
    let all_symbols = get_symbols_if_no_duplicates(&configs)?;
    for config_index in 0..configs.len() {
        if let Err(err) = frb_codegen_multi(&configs, config_index, &all_symbols) {
            error!("fatal: {}", err);
            exit(1);
        }
    }

    info!("Now go and use it :)");
    Ok(())
}

mod tests {
    use std::fs;

    use lazy_static::lazy_static;
    use lib_flutter_rust_bridge_codegen::init_logger;

    lazy_static! {
        static ref LOGGER: () = init_logger(".", true).unwrap();
    }

    // VS Code runs in frb_codegen with "Run test" and flutter_rust_bridge with "Debug test" >_>
    #[allow(dead_code)]
    fn set_dir() {
        if let Ok(metadata) = fs::metadata("frb_codegen") {
            if metadata.is_dir() {
                std::env::set_current_dir("frb_codegen").unwrap();
            }
        }
    }

    #[test]
    fn pure_dart() {
        // `cargo build` can fail because frb_example/pure_dart makes testing a build step! *gasp*
        // If you want to get `cargo build` working again and investigate why pure_dart fails:
        // 1. mv ../frb_example/pure_dart/rust/build.rs ../rb_example/pure_dart/rust/_build.rs
        // 2. Run this test in the debugger - which is a copy of build.rs (but more complete)

        assert!(cfg!(feature = "chrono"));
        assert!(cfg!(feature = "uuid"));

        use lib_flutter_rust_bridge_codegen::{
            config_parse, frb_codegen, get_symbols_if_no_duplicates, RawOpts,
        };
        use std::process::Command;

        set_dir();

        /// Path of input Rust code
        const RUST_INPUT: &str = "../frb_example/pure_dart/rust/src/api.rs";
        /// Path of output generated Dart code
        const DART_OUTPUT: &str = "../frb_example/pure_dart//dart/lib/bridge_generated.dart";

        let _ = *LOGGER;

        // Tell Cargo that if the input Rust code changes, to rerun this build script.
        println!("cargo:rerun-if-changed={RUST_INPUT}");
        // Options for frb_codegen
        let raw_opts = RawOpts {
            // Path of input Rust code
            rust_input: vec![RUST_INPUT.to_string()],
            // Path of output generated Dart code
            dart_output: vec![DART_OUTPUT.to_string()],
            wasm: true,
            dart_decl_output: Some(
                "../frb_example/pure_dart/dart/lib/bridge_definitions.dart".into(),
            ),
            dart_format_line_length: 120,
            // (extra) c output path
            c_output: Some(vec![
                // each field should contain head file name
                "../frb_example/pure_dart/rust/c_output_path/c_output.h".into(),
            ]),
            extra_c_output_path: Some(vec![
                "../frb_example/pure_dart/rust/c_output_path_extra/".into()
            ]),

            // for other options use defaults
            ..Default::default()
        };

        // get opts from raw opts
        let all_configs = config_parse(raw_opts);

        // generation of rust api for ffi (single block)
        let all_symbols = get_symbols_if_no_duplicates(&all_configs).unwrap();
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

    #[test]
    fn pure_dart_multi() {
        // `cargo build` can fail because frb_example/pure_dart makes testing a build step! *gasp*
        // If you want to get `cargo build` working again and investigate why pure_dart fails:
        // 1. mv ../frb_example/pure_dart_multi/rust/build.rs ../rb_example/pure_dart_multi/rust/_build.rs
        // 2. Run this test in the debugger - which is a copy of build.rs (but more complete)

        use lib_flutter_rust_bridge_codegen::{
            config_parse, frb_codegen_multi, get_symbols_if_no_duplicates, RawOpts,
        };
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

        let _ = *LOGGER;

        // Tell Cargo that if the input Rust code changes, to rerun this build script.
        println!("cargo:rerun-if-changed={RUST_INPUT_1}");
        println!("cargo:rerun-if-changed={RUST_INPUT_2}");
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
        let configs = config_parse(raw_opts);

        // generation of rust api for ffi (multi-blocks)
        let all_symbols = get_symbols_if_no_duplicates(&configs).unwrap();
        for config_index in 0..configs.len() {
            frb_codegen_multi(&configs, config_index, &all_symbols).unwrap()
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
