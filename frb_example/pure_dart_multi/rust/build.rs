use lib_flutter_rust_bridge_codegen::{
    config_parse, frb_codegen, get_symbols_if_no_duplicates, RawOpts,
};

/// Path of input Rust code
const RUST_INPUT_1: &str = "src/api_1.rs";
const RUST_INPUT_2: &str = "src/api_2.rs";
/// Path of output generated Dart code
const DART_OUTPUT_1: &str = "../dart/lib/bridge_generated_api_1.dart";
const DART_OUTPUT_2: &str = "../dart/lib/bridge_generated_api_2.dart";
/// Path of output Rust code
const RUST_OUTPUT_1: &str = "src/generated_api_1.rs";
const RUST_OUTPUT_2: &str = "src/generated_api_2.rs";
/// Class name to use in dart, corresponding to each Rust block
const CLASS_NAME_1: &str = "ApiClass1";
const CLASS_NAME_2: &str = "ApiClass2";

fn main() {
    // Tell Cargo that if the input Rust code changes, to rerun this build script.
    println!("cargo:rerun-if-changed={}", RUST_INPUT_1);
    println!("cargo:rerun-if-changed={}", RUST_INPUT_2);
    // Options for frb_codegen
    let raw_opts = RawOpts {
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
        c_output: Some(vec!["macos/Runner/bridge_generated.h".to_string()]),
        // for other options use defaults
        ..Default::default()
    };
    // get opts from raw opts
    let configs = config_parse(raw_opts);

    // generation of rust api for ffi
    let all_symbols = get_symbols_if_no_duplicates(&configs).unwrap();
    for config in configs.iter() {
        frb_codegen(config, &all_symbols).unwrap();
    }
}
