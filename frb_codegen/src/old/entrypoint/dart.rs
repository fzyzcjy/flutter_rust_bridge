use crate::commands::BindgenRustToDartArg;
use crate::others::{
    extract_dart_wire_content, modify_dart_wire_content, sanity_check, DartBasicCode,
    DUMMY_WIRE_CODE_FOR_BINDGEN, EXTRA_EXTERN_FUNC_NAMES,
};
use crate::utils::misc::with_changed_file;
use crate::{command_run, commands, ensure_tools_available, generator, ir, Opts};
use itertools::Itertools;
use log::info;
use pathdiff::diff_paths;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

pub(crate) fn generate_dart_code(
    config: &Opts,
    all_configs: &[Opts],
    ir_file: &ir::IrFile,
    generated_rust: generator::rust::Output,
    all_symbols: &[String],
) -> crate::Result {
    let dart_root = config.dart_root_or_default();
    ensure_tools_available(&dart_root, config.skip_deps_check)?;

    info!("Phase: Generating Dart bindings for Rust");
    // phase-step1: generate temporary c file
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
            .map_err(Into::into)
        },
    )?;

    let effective_func_names = [
        generated_rust.extern_func_names,
        EXTRA_EXTERN_FUNC_NAMES.to_vec(),
    ]
    .concat();

    for (i, each_path) in config.c_output_path.iter().enumerate() {
        let c_dummy_code =
            generator::c::generate_dummy(config, all_configs, &effective_func_names, i);
        println!("the path is {each_path:?}");
        fs::create_dir_all(Path::new(each_path).parent().unwrap())?;
        fs::write(
            each_path,
            fs::read_to_string(&temp_bindgen_c_output_file)? + "\n" + &c_dummy_code,
        )?;
    }

    // phase-step2: generate raw dart code instance from the c file
    let generated_dart_wire_code_raw = fs::read_to_string(temp_dart_wire_file)?;
    let generated_dart_wire = extract_dart_wire_content(&modify_dart_wire_content(
        &generated_dart_wire_code_raw,
        &config.dart_wire_class_name(),
    ));
    sanity_check(&generated_dart_wire.body, &config.dart_wire_class_name())?;

    // phase-step3: compose dart codes and write to file
    let generated_dart = ir_file.generate_dart(config, &generated_rust.wasm_exports);
    let generated_dart_decl_all = &generated_dart.decl_code;
    let generated_dart_impl_io_wire = &generated_dart.impl_code.io + &generated_dart_wire;

    let dart_output_paths = config.get_dart_output_paths();
    let dart_output_dir = Path::new(&dart_output_paths.base_path).parent().unwrap();
    fs::create_dir_all(dart_output_dir)?;

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
            &dart_output_paths.base_path,
            (&generated_dart.file_prelude
                + generated_dart_decl_all
                + &generated_dart.impl_code.common)
                .to_text(),
        )?;
        fs::write(
            &dart_output_paths.io_path,
            (&generated_dart.file_prelude + &generated_dart_impl_io_wire).to_text(),
        )?;
        fs::write(
            &dart_output_paths.wasm_path,
            (&generated_dart.file_prelude + &generated_dart.impl_code.wasm).to_text(),
        )?;
    } else {
        let mut out = generated_dart.file_prelude
            + generated_dart_decl_all
            + &generated_dart.impl_code.common
            + &generated_dart_impl_io_wire;
        out.import = out.import.lines().unique().join("\n");
        fs::write(&dart_output_paths.base_path, out.to_text())?;
    }

    info!("Phase: Running build_runner");
    let dart_root = &config.dart_root;
    if generated_dart.needs_freezed && config.build_runner {
        let dart_root = dart_root
            .as_ref()
            .ok_or(crate::config::Error::FailedInferDartRoot)?;
        commands::build_runner(dart_root)?;
    }

    info!("Phase: Formatting Dart code");
    command_run!(
        commands::format_dart[config.dart_format_line_length],
        &dart_output_paths.base_path,
        ?config.dart_decl_output_path,
        (
            config.wasm_enabled,
            dart_output_paths.wasm_path,
            dart_output_paths.io_path,
        ),
        (
            generated_dart.needs_freezed && config.build_runner,
            config.dart_freezed_path(),
        )
    )?;

    Ok(())
}

fn write_dart_decls(
    config: &Opts,
    dart_decl_output_path: &str,
    dart_output_dir: &Path,
    generated_dart: &crate::generator::dart::Output,
    generated_dart_decl_all: &DartBasicCode,
    generated_dart_impl_io_wire: &DartBasicCode,
) -> crate::Result {
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

    let dart_output_paths = config.get_dart_output_paths();
    if config.wasm_enabled {
        fs::write(
            &dart_output_paths.base_path,
            (&generated_dart.file_prelude + &impl_import_decl + &generated_dart.impl_code.common)
                .to_text(),
        )?;
        fs::write(
            dart_output_paths.io_path,
            (&generated_dart.file_prelude + &impl_import_decl + generated_dart_impl_io_wire)
                .to_text(),
        )?;
        fs::write(
            dart_output_paths.wasm_path,
            (&generated_dart.file_prelude + &impl_import_decl + &generated_dart.impl_code.wasm)
                .to_text(),
        )?;
    } else {
        fs::write(
            &dart_output_paths.base_path,
            (&generated_dart.file_prelude
                + &impl_import_decl
                + &generated_dart.impl_code.common
                + generated_dart_impl_io_wire)
                .to_text(),
        )?;
    }
    Ok(())
}
