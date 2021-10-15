use std::fs;

use env_logger::Env;
use log::{debug, info};
use structopt::StructOpt;

use crate::config::RawOpts;
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

    let config = config::parse(RawOpts::from_args());
    info!("Picked config: {:?}", &config);

    info!("Phase: Parse source code to AST");
    let source_rust_content = fs::read_to_string(&config.rust_input_path).unwrap();
    let file_ast = syn::parse_file(&source_rust_content).unwrap();

    info!("Phase: Parse AST to IR");
    let raw_api_file = parser::parse(&source_rust_content, file_ast);
    debug!("parsed functions: {:?}", &raw_api_file);

    info!("Phase: Transform IR");
    let api_file = transformer::transform(raw_api_file);
    debug!("transformed functions: {:?}", &api_file);

    info!("Phase: Generate Rust code");
    let generated_rust = generator_rust::generate(&api_file, &path_stem(&config.rust_input_path));
    fs::write(&config.rust_output_path, generated_rust.code).unwrap();

    info!("Phase: Generate Dart code");
    let generated_dart_api = generator_dart::generate(
        &api_file,
        &config.dart_api_class_name(),
        &config.dart_api_impl_class_name(),
        &config.dart_wire_class_name(),
    );

    info!("Phase: Other things");

    commands::format_rust(&config.rust_output_path);

    others::try_add_mod_to_lib(&config.rust_crate_dir, &config.rust_output_path);

    let temp_dart_wire_file = tempfile::NamedTempFile::new().unwrap();
    let temp_dart_wire_path = temp_dart_wire_file.path().as_os_str().to_str().unwrap();
    with_changed_file(
        &config.rust_output_path,
        DUMMY_WIRE_CODE_FOR_BINDGEN,
        || {
            commands::bindgen_rust_to_dart(
                &config.rust_crate_dir,
                &config.c_output_path,
                temp_dart_wire_path,
                &config.dart_wire_class_name(),
            );
        },
    );
    let generated_dart_wire_code_raw = fs::read_to_string(temp_dart_wire_file).unwrap();
    let (generated_dart_wire_import_code, generated_dart_wire_body_code) =
        extract_dart_wire_content(&modify_dart_wire_content(
            &generated_dart_wire_code_raw,
            &config.dart_wire_class_name(),
        ));

    sanity_check(
        &generated_dart_wire_body_code,
        &config.dart_wire_class_name(),
    );

    let generated_dart_code = format!(
        "{}\n{}\n{}\n{}\n{}",
        generated_dart_api.header,
        generated_dart_wire_import_code,
        generated_dart_api.api_class,
        generated_dart_api.other,
        generated_dart_wire_body_code,
    );
    fs::write(&config.dart_output_path, generated_dart_code).unwrap();
    commands::format_dart(&config.dart_output_path, config.dart_format_line_length);

    info!("Success! Now go and use it :)");
}
