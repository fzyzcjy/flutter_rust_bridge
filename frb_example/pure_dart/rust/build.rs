use lib_flutter_rust_bridge_codegen::{
    config_parse, frb_codegen, get_symbols_if_no_duplicates, RawOpts,
};

/// Path of input Rust code
const RUST_INPUT: &str = "src/api.rs";
/// Path of output generated Dart code
const DART_OUTPUT: &str = "../dart/lib/bridge_generated.dart";

fn main() {
    // Tell Cargo that if the input Rust code changes, to rerun this build script.
    println!("cargo:rerun-if-changed={}", RUST_INPUT);
    // Options for frb_codegen
    let mut raw_opts = RawOpts {
        // Path of input Rust code
        rust_input: vec![RUST_INPUT.to_string()],
        // Path of output generated Dart code
        dart_output: vec![DART_OUTPUT.to_string()],
        wasm: true,
        dart_decl_output: Some("../dart/lib/bridge_definitions.dart".into()),
        dart_format_line_length: 120,
        // for other options use defaults
        ..Default::default()
    };

    if cfg!(feature = "c-output") {
        raw_opts.c_output = Some(vec!["./c_output_path/c_output.h".into()]);
    }

    if cfg!(feature = "extra-c-output-path") {
        raw_opts.extra_c_output_path = Some(vec![
            // For test, the below 2 paths format are made a little different
            "./extra_c_output_path_1/".into(),
            "extra_c_output_path_2".into(),
        ]);
    }
    // get opts from raw opts
    let configs = config_parse(raw_opts);

    // generation of rust api for ffi
    let all_symbols = get_symbols_if_no_duplicates(&configs).unwrap();
    for config in configs.iter() {
        frb_codegen(config, &all_symbols).unwrap();
    }
}
