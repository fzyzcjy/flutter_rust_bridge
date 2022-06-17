use lib_flutter_rust_bridge_codegen::{config_parse, frb_codegen, RawOpts};

/// Path of input Rust code
const RUST_INPUT: &str = "src/api.rs";
/// Path of output generated Dart code
const DART_OUTPUT: &str = "../dart/lib/bridge_generated.dart";

fn main() {
    // Tell Cargo that if the input Rust code changes, to rerun this build script.
    println!("cargo:rerun-if-changed={}", RUST_INPUT);
    // Options for frb_codegen
    let raw_opts = RawOpts {
        // Path of input Rust code
        rust_input: vec![RUST_INPUT.to_string()],
        // Path of output generated Dart code
        dart_output: vec![DART_OUTPUT.to_string()],
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
