use lib_lutter_rust_bridge_codegen::{frb_codegen, Opts};

const BRIDGE_FILE: &str = "src/api.rs";

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed={BRIDGE_FILE}");
    // settings for fbr_codegen
    let opts = Opts {
        // Path of input Rust code
        rust_input: BRIDGE_FILE.to_string(),
        // Path of output generated Dart code
        dart_output: "../dart/lib/bridge_generated.dart".to_string(),
        // for other options lets use default
        ..Default::default()
    };
    // run fbr_codegen
    frb_codegen(opts).unwrap();
}
