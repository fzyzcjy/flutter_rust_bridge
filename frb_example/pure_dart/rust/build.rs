use lib_flutter_rust_bridge_codegen::{config_parse, frb_codegen, RawOpts};

/// Path of input Rust code
const RUST_INPUT_1: &str = "src/api_1.rs";
const RUST_INPUT_2: &str = "src/api_2.rs";
/// Path of output generated Dart code
const DART_OUTPUT_1: &str = "../dart/lib/bridge_generated_1.dart";
const DART_OUTPUT_2: &str = "../dart/lib/bridge_generated_2.dart";
/// Path of output Rust code
const RUST_OUTPUT_1: &str = "src/generated_api_1.rs";
const RUST_OUTPUT_2: &str = "src/generated_api_2.rs";
/// Class name of each Rust block of api
const CLASS_NAME_1: &str = "class_1";
const CLASS_NAME_2: &str = "class_2";

fn main() {
    // NOTE: the following 2 tests can't be run at the same time
    // since similiar single rust block api test would be executed in other github CLI,
    // I take `test_multi_block_of_api` here

    // test_single_block_of_api();
    test_multi_block_of_api();
}

#[allow(unused)]
fn test_single_block_of_api() {
    // Tell Cargo that if the input Rust code changes, to rerun this build script.
    println!("cargo:rerun-if-changed={}", RUST_INPUT_1);
    // Options for frb_codegen
    let raw_opts = RawOpts {
        // Path of input Rust code
        rust_input: vec![RUST_INPUT_1.to_string()],
        // Path of output generated Dart code
        dart_output: vec![DART_OUTPUT_1.to_string()],
        // for other options use defaults
        ..Default::default()
    };
    // get opts from raw opts
    let opts = config_parse(raw_opts);
    // run flutter_rust_bridge_codegen
    for opt in &opts {
        frb_codegen(opt).unwrap();
    }
}

#[allow(unused)]
fn test_multi_block_of_api() {
    // Tell Cargo that if the input Rust code changes, to rerun this build script.
    println!("cargo:rerun-if-changed={}", RUST_INPUT_1);
    println!("cargo:rerun-if-changed={}", RUST_INPUT_2);
    // Options for frb_codegen
    let raw_opts = RawOpts {
        // Path of input Rust code
        rust_input: vec![RUST_INPUT_1.to_string(), RUST_INPUT_2.to_string()],
        // Path of output generated Dart code
        dart_output: vec![DART_OUTPUT_1.to_string(),DART_OUTPUT_2.to_string()],
        // Path of output Rust code
        rust_output: Some(vec![RUST_OUTPUT_1.to_string(), RUST_OUTPUT_2.to_string()]),
        // Class name of each Rust block of api
        class_name: Some(vec![CLASS_NAME_1.to_string(), CLASS_NAME_2.to_string()]),
        // for other options use defaults
        ..Default::default()
    };
    // get opts from raw opts
    let opts = config_parse(raw_opts);
    // run flutter_rust_bridge_codegen
    for opt in &opts {
        frb_codegen(opt).unwrap();
    }
}
