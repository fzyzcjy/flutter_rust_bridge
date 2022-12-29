//! Main documentation is in https://github.com/fzyzcjy/flutter_rust_bridge
#![allow(clippy::vec_init_then_push)]
// #![warn(clippy::wildcard_enum_match_arm)]

use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use itertools::Itertools;
use log::info;
use pathdiff::diff_paths;

use crate::commands::BindgenRustToDartArg;
use crate::others::*;
use crate::target::Acc;
use crate::target::Target;
use crate::utils::*;

mod config;
mod tools;

pub use crate::commands::ensure_tools_available;
pub use crate::config::parse as config_parse;
pub use crate::config::Opts;
pub use crate::config::RawOpts;
pub use crate::utils::get_symbols_if_no_duplicates;

#[macro_use]
mod commands;
mod consts;
mod error;
mod generator;
mod ir;
mod markers;
mod method_utils;
mod others;
mod parser;
mod source_graph;
mod target;
mod transformer;
mod utils;
use error::*;

pub fn frb_codegen(config: &config::Opts, all_symbols: &[String]) -> anyhow::Result<()> {
    let dart_root = config.dart_root_or_default();
    ensure_tools_available(&dart_root, config.skip_deps_check)?;

    info!("Picked config: {:?}", config);

    let rust_output_dir = Path::new(&config.rust_output_path).parent().unwrap();
    let dart_output_dir = Path::new(&config.dart_output_path).parent().unwrap();

    info!("Phase: Parse source code to AST, then to IR");
    let raw_ir_file = config.get_ir_file()?;

    info!("Phase: Transform IR");
    let ir_file = transformer::transform(raw_ir_file);

    info!("Phase: Generate Rust code");
    fs::create_dir_all(rust_output_dir)?;
    let generated_rust = ir_file.generate_rust(config);
    write_rust_modules(config, &generated_rust)?;

    info!("Phase: Generate Dart code");
    let generated_dart = ir_file.generate_dart(config, &generated_rust.wasm_exports);

    run!(
        commands::format_rust,
        &config.rust_output_path,
        (
            config.wasm_enabled && !config.inline_rust,
            config.rust_io_output_path(),
            config.rust_wasm_output_path(),
        )
    )?;

    if !config.skip_add_mod_to_lib {
        others::try_add_mod_to_lib(&config.rust_crate_dir, &config.rust_output_path);
    }

    info!("Phase: Generating Dart bindings for Rust");
    let temp_dart_wire_file = tempfile::NamedTempFile::new()?;
    let temp_bindgen_c_output_file = tempfile::Builder::new().suffix(".h").tempfile()?;
    let exclude_symbols = generated_rust.get_exclude_symbols(all_symbols);
    with_changed_file(
        &config.rust_output_path,
        DUMMY_WIRE_CODE_FOR_BINDGEN,
        || {
            commands::bindgen_rust_to_dart(
                BindgenRustToDartArg {
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
                },
                &dart_root,
            )
        },
    )?;

    let effective_func_names = [
        generated_rust.extern_func_names,
        EXTRA_EXTERN_FUNC_NAMES.to_vec(),
    ]
    .concat();

    let c_dummy_code = generator::c::generate_dummy(&effective_func_names);
    for each_path in config.c_output_path.iter() {
        println!("the path is {:?}", each_path);
        fs::create_dir_all(Path::new(each_path).parent().unwrap())?;
        fs::write(
            each_path,
            fs::read_to_string(&temp_bindgen_c_output_file)? + "\n" + &c_dummy_code,
        )?;
    }

    fs::create_dir_all(dart_output_dir)?;
    let generated_dart_wire_code_raw = fs::read_to_string(temp_dart_wire_file)?;
    let generated_dart_wire = extract_dart_wire_content(&modify_dart_wire_content(
        &generated_dart_wire_code_raw,
        &config.dart_wire_class_name(),
    ));

    sanity_check(&generated_dart_wire.body, &config.dart_wire_class_name())?;

    let generated_dart_decl_all = &generated_dart.decl_code;
    let generated_dart_impl_io_wire = &generated_dart.impl_code.io + &generated_dart_wire;
    if let Some(dart_decl_output_path) = &config.dart_decl_output_path {
        write_dart_decls(
            config,
            dart_decl_output_path,
            dart_output_dir,
            &generated_dart,
            generated_dart_decl_all,
            &generated_dart_impl_io_wire,
        )?;
    } else if config.wasm_enabled {
        fs::write(
            &config.dart_output_path,
            (&generated_dart.file_prelude
                + generated_dart_decl_all
                + &generated_dart.impl_code.common)
                .to_text(),
        )?;
        fs::write(
            config.dart_io_output_path(),
            (&generated_dart.file_prelude + &generated_dart_impl_io_wire).to_text(),
        )?;
        fs::write(
            config.dart_wasm_output_path(),
            (&generated_dart.file_prelude + &generated_dart.impl_code.wasm).to_text(),
        )?;
    } else {
        let mut out = generated_dart.file_prelude
            + generated_dart_decl_all
            + &generated_dart.impl_code.common
            + &generated_dart_impl_io_wire;
        out.import = out.import.lines().unique().join("\n");
        fs::write(&config.dart_output_path, out.to_text())?;
    }

    info!("Phase: Running build_runner");
    let dart_root = &config.dart_root;
    if generated_dart.needs_freezed && config.build_runner {
        let dart_root = dart_root.as_ref().ok_or_else(|| {
            Error::str(
                "build_runner configured to run, but Dart root could not be inferred.
        Please specify --dart-root, or disable build_runner with --no-build-runner.",
            )
        })?;
        commands::build_runner(dart_root)?;
    }

    info!("Phase: Formatting Dart code");
    run!(
        commands::format_dart[config.dart_format_line_length],
        &config.dart_output_path,
        ?config.dart_decl_output_path,
        (
            config.wasm_enabled,
            config.dart_wasm_output_path(),
            config.dart_io_output_path(),
        ),
        (
            generated_dart.needs_freezed && config.build_runner,
            config.dart_freezed_path(),
        )
    )?;

    info!("Success!");
    Ok(())
}

fn write_dart_decls(
    config: &Opts,
    dart_decl_output_path: &str,
    dart_output_dir: &Path,
    generated_dart: &crate::generator::dart::Output,
    generated_dart_decl_all: &DartBasicCode,
    generated_dart_impl_io_wire: &DartBasicCode,
) -> anyhow::Result<()> {
    let impl_import_decl = DartBasicCode {
        import: format!(
            "import \"{}\";",
            diff_paths(dart_decl_output_path, dart_output_dir)
                .unwrap()
                .to_str()
                .unwrap()
        ),
        ..Default::default()
    };

    let common_import = DartBasicCode {
        import: if config.wasm_enabled {
            format!(
                "import '{}' if (dart.library.html) '{}';",
                config
                    .dart_io_output_path()
                    .file_name()
                    .and_then(OsStr::to_str)
                    .unwrap(),
                config
                    .dart_wasm_output_path()
                    .file_name()
                    .and_then(OsStr::to_str)
                    .unwrap(),
            )
        } else {
            "".into()
        },
        ..Default::default()
    };

    fs::write(
        dart_decl_output_path,
        (&generated_dart.file_prelude + &common_import + generated_dart_decl_all).to_text(),
    )?;
    if config.wasm_enabled {
        fs::write(
            &config.dart_output_path,
            (&generated_dart.file_prelude + &impl_import_decl + &generated_dart.impl_code.common)
                .to_text(),
        )?;
        fs::write(
            config.dart_io_output_path(),
            (&generated_dart.file_prelude + &impl_import_decl + generated_dart_impl_io_wire)
                .to_text(),
        )?;
        fs::write(
            config.dart_wasm_output_path(),
            (&generated_dart.file_prelude + &impl_import_decl + &generated_dart.impl_code.wasm)
                .to_text(),
        )?;
    } else {
        fs::write(
            &config.dart_output_path,
            (&generated_dart.file_prelude
                + &impl_import_decl
                + &generated_dart.impl_code.common
                + generated_dart_impl_io_wire)
                .to_text(),
        )?;
    }
    Ok(())
}

fn write_rust_modules(
    config: &Opts,
    generated_rust: &crate::generator::rust::Output,
) -> anyhow::Result<()> {
    let Acc { common, io, wasm } = &generated_rust.code;
    fn emit_platform_module(name: &str, body: &str, config: &Opts, target: Target) -> String {
        if config.inline_rust {
            format!("mod {} {{ use super::*;\n {} }}", name, body)
        } else {
            let path = match target {
                Target::Io => config.rust_io_output_path(),
                Target::Wasm => config.rust_wasm_output_path(),
                _ => panic!("unsupported target: {:?}", target),
            };
            let path = path.file_name().and_then(OsStr::to_str).unwrap();
            format!("#[path = \"{}\"] mod {};", path, name)
        }
    }
    let common = format!(
        "{}
{mod_web}

#[cfg(not(target_family = \"wasm\"))]
{mod_io}
#[cfg(not(target_family = \"wasm\"))]
pub use io::*;
",
        common,
        mod_web = if config.wasm_enabled {
            format!(
                "
/// cbindgen:ignore
#[cfg(target_family = \"wasm\")]
{}
#[cfg(target_family = \"wasm\")]
pub use web::*;",
                emit_platform_module("web", wasm, config, Target::Wasm)
            )
        } else {
            "".into()
        },
        mod_io = emit_platform_module("io", io, config, Target::Io),
    );
    fs::write(&config.rust_output_path, common)?;
    if !config.inline_rust {
        fs::write(
            config.rust_io_output_path(),
            format!("use super::*;\n{}", io),
        )?;
        if config.wasm_enabled {
            fs::write(
                config.rust_wasm_output_path(),
                format!("use super::*;\n{}", wasm),
            )?;
        }
    }
    Ok(())
}
