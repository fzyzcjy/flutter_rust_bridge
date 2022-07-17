//! Main documentation is in https://github.com/fzyzcjy/flutter_rust_bridge

use std::fs;
use std::path::Path;

use log::info;
use pathdiff::diff_paths;

use crate::commands::BindgenRustToDartArg;
use crate::others::*;
use crate::utils::*;

mod config;

pub use crate::commands::ensure_tools_available;
pub use crate::config::parse as config_parse;
pub use crate::config::Opts;
pub use crate::config::RawOpts;
pub use crate::utils::get_symbols_if_no_duplicates;

mod commands;
mod error;
mod generator;
mod ir;
mod markers;
mod method_utils;
mod others;
mod parser;
mod source_graph;
mod transformer;
mod utils;
use error::*;

pub fn frb_codegen(config: &config::Opts, all_symbols: &[String]) -> anyhow::Result<()> {
    ensure_tools_available()?;

    info!("Picked config: {:?}", config);

    let rust_output_dir = Path::new(&config.rust_output_path).parent().unwrap();
    let dart_output_dir = Path::new(&config.dart_output_path).parent().unwrap();

    info!("Phase: Parse source code to AST, then to IR");
    let raw_ir_file = config.get_ir_file();

    info!("Phase: Transform IR");
    let ir_file = transformer::transform(raw_ir_file);

    info!("Phase: Generate Rust code");
    let generated_rust = ir_file.generate_rust(config);
    let exclude_symbols = generated_rust.get_exclude_symbols(all_symbols);
    fs::create_dir_all(&rust_output_dir)?;
    fs::write(&config.rust_output_path, generated_rust.code)?;

    info!("Phase: Generate Dart code");
    let (generated_dart, needs_freezed) = ir_file.generate_dart(config)?;

    info!("Phase: Other things");

    commands::format_rust(&config.rust_output_path)?;

    if !config.skip_add_mod_to_lib {
        others::try_add_mod_to_lib(&config.rust_crate_dir, &config.rust_output_path);
    }

    let temp_dart_wire_file = tempfile::NamedTempFile::new()?;
    let temp_bindgen_c_output_file = tempfile::Builder::new().suffix(".h").tempfile()?;
    with_changed_file(
        &config.rust_output_path,
        DUMMY_WIRE_CODE_FOR_BINDGEN,
        || {
            commands::bindgen_rust_to_dart(BindgenRustToDartArg {
                rust_crate_dir: &config.rust_crate_dir,
                c_output_path: temp_bindgen_c_output_file
                    .path()
                    .as_os_str()
                    .to_str()
                    .unwrap(),
                dart_output_path: temp_dart_wire_file.path().as_os_str().to_str().unwrap(),
                dart_class_name: &config.dart_wire_class_name(),
                c_struct_names: ir_file.get_c_struct_names(),
                exclude_symbols,
                llvm_install_path: &config.llvm_path[..],
                llvm_compiler_opts: &config.llvm_compiler_opts,
            })
        },
    )?;

    let effective_func_names = [
        generated_rust.extern_func_names,
        EXTRA_EXTERN_FUNC_NAMES.to_vec(),
    ]
    .concat();
    let c_dummy_code = generator::c::generate_dummy(&effective_func_names);
    for output in &config.c_output_path {
        fs::create_dir_all(Path::new(output).parent().unwrap())?;
        fs::write(
            &output,
            fs::read_to_string(&temp_bindgen_c_output_file)? + "\n" + &c_dummy_code,
        )?;
    }

    fs::create_dir_all(&dart_output_dir)?;
    let generated_dart_wire_code_raw = fs::read_to_string(temp_dart_wire_file)?;
    let generated_dart_wire = extract_dart_wire_content(&modify_dart_wire_content(
        &generated_dart_wire_code_raw,
        &config.dart_wire_class_name(),
    ));

    sanity_check(&generated_dart_wire.body, &config.dart_wire_class_name())?;

    let generated_dart_decl_all = generated_dart.decl_code;
    let generated_dart_impl_all = &generated_dart.impl_code + &generated_dart_wire;
    if let Some(dart_decl_output_path) = &config.dart_decl_output_path {
        let impl_import_decl = DartBasicCode {
            import: format!(
                "import \"{}\";",
                diff_paths(dart_decl_output_path, dart_output_dir)
                    .unwrap()
                    .to_str()
                    .unwrap()
            ),
            part: String::new(),
            body: String::new(),
        };
        fs::write(
            &dart_decl_output_path,
            (&generated_dart.file_prelude + &generated_dart_decl_all).to_text(),
        )?;
        fs::write(
            &config.dart_output_path,
            (&generated_dart.file_prelude + &impl_import_decl + &generated_dart_impl_all).to_text(),
        )?;
    } else {
        fs::write(
            &config.dart_output_path,
            (&generated_dart.file_prelude + &generated_dart_decl_all + &generated_dart_impl_all)
                .to_text(),
        )?;
    }

    let dart_root = &config.dart_root;
    if needs_freezed && config.build_runner {
        let dart_root = dart_root.as_ref().ok_or_else(|| {
            Error::str(
                "build_runner configured to run, but Dart root could not be inferred.
        Please specify --dart-root, or disable build_runner with --no-build-runner.",
            )
        })?;
        commands::build_runner(dart_root)?;
        commands::format_dart(
            &config
                .dart_output_freezed_path()
                .ok_or_else(|| Error::str("Invalid freezed file path"))?,
            config.dart_format_line_length,
        )?;
    }

    commands::format_dart(&config.dart_output_path, config.dart_format_line_length)?;
    if let Some(dart_decl_output_path) = &config.dart_decl_output_path {
        commands::format_dart(dart_decl_output_path, config.dart_format_line_length)?;
    }

    info!("Success!");
    Ok(())
}
