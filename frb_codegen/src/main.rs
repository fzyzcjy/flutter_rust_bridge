use std::fs;

use env_logger::Env;
use log::{debug, info};

use crate::others::*;
use crate::utils::*;

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

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config_path = parse_command_line_args();
    let config = config::RawOpts::read(&config_path);

    info!("Phase: Parse source code to AST");
    let source_rust_content = fs::read_to_string(&config.rust.input_path).unwrap();
    let file_ast = syn::parse_file(&source_rust_content).unwrap();

    info!("Phase: Parse AST to IR");
    let raw_api_file = parser::parse(file_ast);
    debug!("parsed functions: {:?}", &raw_api_file);

    info!("Phase: Transform IR");
    let api_file = transformer::transform(raw_api_file);
    debug!("transformed functions: {:?}", &api_file);

    info!("Phase: Generate Rust code");
    let generated_rust_code =
        generator_rust::generate(&api_file, &path_stem(&config.rust.input_path));
    fs::write(&config.rust.output_path, generated_rust_code).unwrap();

    info!("Phase: Generate Dart code");
    let (
        generated_dart_api_header_code,
        generated_dart_api_api_class_code,
        generated_dart_api_other_code,
    ) = generator_dart::generate(
        &api_file,
        &config.dart.api_class_name(),
        &config.dart.api_impl_class_name(),
        &config.dart.wire_class_name(),
    );

    info!("Phase: Other things");

    commands::format_rust(&config.rust.output_path);

    let temp_dart_wire_file = tempfile::NamedTempFile::new().unwrap();
    let temp_dart_wire_path = temp_dart_wire_file.path().as_os_str().to_str().unwrap();
    with_changed_file(
        &config.rust.output_path,
        DUMMY_WIRE_CODE_FOR_BINDGEN,
        || {
            commands::bindgen_rust_to_dart(
                &config.rust.crate_dir,
                &config.c.output_path,
                temp_dart_wire_path,
                &config.dart.wire_class_name(),
            );
        },
    );
    let generated_dart_wire_code_raw = fs::read_to_string(temp_dart_wire_file).unwrap();
    let (generated_dart_wire_import_code, generated_dart_wire_body_code) =
        extract_dart_wire_content(&modify_dart_wire_content(
            &generated_dart_wire_code_raw,
            &config.dart.wire_class_name(),
        ));

    sanity_check(
        &generated_dart_wire_body_code,
        &config.dart.wire_class_name(),
    );

    let generated_dart_code = format!(
        "{}\n{}\n{}\n{}\n{}",
        generated_dart_api_header_code,
        generated_dart_wire_import_code,
        generated_dart_api_api_class_code,
        generated_dart_api_other_code,
        generated_dart_wire_body_code,
    );
    fs::write(&config.dart.output_path, generated_dart_code).unwrap();
    commands::format_dart(&config.dart.output_path, config.dart.format_line_length);

    info!("Success! Now go and use it :)");
}
