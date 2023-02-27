use lib_flutter_rust_bridge_codegen::{
    config_parse, frb_codegen, get_symbols_if_no_duplicates, init_logger, RawOpts,
};

/// Path of input Rust code
const RUST_INPUT: &str = "src/api.rs";
/// Path of output generated Dart code
const DART_OUTPUT: &str = "../dart/lib/bridge_generated.dart";

fn main() {
    init_logger("./logs/", true).unwrap();

    // Tell Cargo that if the input Rust code changes, to rerun this build script.
    println!("cargo:rerun-if-changed={RUST_INPUT}");
    // Options for frb_codegen
    let raw_opts = RawOpts {
        // Path of input Rust code
        rust_input: vec![RUST_INPUT.to_string()],
        // Path of output generated Dart code
        dart_output: vec![DART_OUTPUT.to_string()],
        wasm: true,
        dart_decl_output: Some("../dart/lib/bridge_definitions.dart".into()),
        dart_format_line_length: 120,
        // (extra) c output path
        c_output: Some(vec![
            // each field should contain head file name
            "./c_output_path/c_output.h".into(),
        ]),
        extra_c_output_path: Some(vec!["./c_output_path_extra/".into()]),

        // for other options use defaults
        ..Default::default()
    };

    // get opts from raw opts
    let all_configs = config_parse(raw_opts);

    // generation of rust api for ffi (single block)
    let all_symbols = get_symbols_if_no_duplicates(&all_configs).unwrap();
    assert_eq!(all_configs.len(), 1);
    frb_codegen(&all_configs[0], &all_symbols).unwrap();
}
