mod api_types;
mod commands;
mod config;
mod generator_common;
mod generator_dart;
mod generator_rust;
mod others;
mod parser;
mod transformer;
mod utils;

use crate::others::*;
use crate::utils::*;
use env_logger::Env;
use log::{debug, info};
use std::fs;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config_path = parse_command_line_args();
    let config = config::Config::read(&config_path);

    info!("Phase: Parse source code to AST");
    let source_rust_content = fs::read_to_string(&config.rust.api_path).unwrap();
    let file_ast = syn::parse_file(&source_rust_content).unwrap();

    info!("Phase: Parse AST to IR");
    let raw_api_file = parser::parse(file_ast);
    debug!("parsed functions: {:?}", &raw_api_file);

    info!("Phase: Transform IR");
    let api_file = transformer::transform(raw_api_file);
    debug!("transformed functions: {:?}", &api_file);

    info!("Phase: Generate Rust code");
    let generated_rust_code =
        generator_rust::generate(&api_file, &path_stem(&config.rust.api_path));
    fs::write(&config.rust.wire_path, generated_rust_code).unwrap();

    info!("Phase: Generate Dart code");
    let generated_dart_code = generator_dart::generate(
        &api_file,
        &config.dart.api_class_name,
        &config.dart.wire_class_name,
        &path_filename(&config.dart.wire_path),
    );
    fs::write(&config.dart.api_path, generated_dart_code).unwrap();

    info!("Phase: Other things");

    commands::format_rust(&config.rust.wire_path);
    commands::format_dart(&config.dart.api_path, config.dart.format_line_length);

    with_changed_file(&config.rust.wire_path, DUMMY_WIRE_CODE_FOR_BINDGEN, || {
        commands::bindgen_rust_to_dart(
            &config.rust.crate_dir,
            &config.c.wire_path,
            &config.dart.wire_path,
            &config.dart.wire_class_name,
        );
    });

    sanity_check(&config);

    modify_dart_wire_content(&config.dart.wire_path, &config.dart.wire_class_name);
    commands::format_dart(&config.dart.wire_path, config.dart.format_line_length);

    info!("Success! Now go and use it :)");
}
