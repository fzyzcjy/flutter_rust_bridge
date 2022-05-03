use lib_flutter_rust_bridge_codegen::{frb_codegen, Opts};

/// Path of input Rust code
const RUST_INPUT: &str = "src/api.rs";
/// Path of output generated Dart code
const DART_OUTPUT: &str = "../dart/lib/bridge_generated.dart";

fn main() {
    // Tell Cargo that if the input Rust code changes, to rerun this build script.
    println!("cargo:rerun-if-changed={}", RUST_INPUT);
    // Options for frb_codegen
    let opts = Opts {
        // Path of input Rust code
        rust_input: RUST_INPUT.to_string(),
        // Path of output generated Dart code
        dart_output: DART_OUTPUT.to_string(),
        // for other options use defaults
        ..Default::default()
    };
    // run flutter_rust_bridge_codegen
    frb_codegen(opts).unwrap();
}
