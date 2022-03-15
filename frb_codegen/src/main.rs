use std::fs;
use std::path::Path;

use env_logger::Env;
use log::{debug, info};
use pathdiff::diff_paths;
use structopt::StructOpt;

use crate::commands::ensure_tools_available;
use crate::config::RawOpts;
use crate::generator::rust::Code;
use crate::ir::*;
use crate::others::*;
use crate::utils::*;

mod commands;
mod config;
mod generator;
mod ir;
mod markers;
mod others;
mod parser;
mod source_graph;
mod transformer;
mod utils;

fn main() -> anyhow::Result<()> {
    let raw_opts = RawOpts::from_args();
    env_logger::Builder::from_env(Env::default().default_filter_or(if raw_opts.verbose {
        "debug"
    } else {
        "info"
    }))
    .init();

    ensure_tools_available();

    let config = config::parse(raw_opts);
    info!("Picked config: {:#?}", &config);

    let dart_output_dir = Path::new(&config.dart_output_path).parent().unwrap();

    info!("Phase: Parse source code to AST");
    let source_rust_content = fs::read_to_string(&config.rust_input_path)?;
    let file_ast = syn::parse_file(&source_rust_content)?;

    info!("Phase: Parse AST to IR");
    let raw_ir_file = parser::parse(&source_rust_content, file_ast, &config.manifest_path);
    debug!("parsed functions: {:?}", &raw_ir_file);

    info!("Phase: Transform IR");
    let ir_file = transformer::transform(raw_ir_file);
    debug!("transformed functions: {:?}", &ir_file);

    info!("Phase: Generate Rust code");
    let generated_rust = generator::rust::generate(
        &ir_file,
        &mod_from_rust_path(&config.rust_input_path, &config.rust_crate_dir),
        config.wasm,
    );
    match generated_rust.code {
        Code::Native(native) => {
            fs::write(&config.rust_native_output_path().unwrap(), native)?;
        }
        Code::Multi { native, wasm, stub } => {
            fs::write(&config.rust_output_path, stub)?;
            fs::create_dir_all(config.rust_multi_dir().unwrap())?;
            fs::write(&config.rust_native_output_path().unwrap(), native)?;
            fs::write(&config.rust_wasm_output_path().unwrap(), wasm)?;
        }
    }

    info!("Phase: Generate Dart code");
    let (generated_dart, needs_freezed) = generator::dart::generate(
        &ir_file,
        &config.dart_api_class_name(),
        &config.dart_api_impl_class_name(),
        &config.dart_wire_class_name(),
        config
            .dart_output_path_name()
            .expect("Invalid dart_output_path_name"),
        config.wasm,
    );

    info!("Phase: Other things");

    commands::format_rust(&config.rust_output_path);
    if config.wasm {
        commands::format_rust(config.rust_wasm_output_path().unwrap().to_str().unwrap());
        commands::format_rust(config.rust_native_output_path().unwrap().to_str().unwrap());
    }

    if !config.skip_add_mod_to_lib {
        others::try_add_mod_to_lib(&config.rust_crate_dir, &config.rust_output_path);
    }

    let c_struct_names = ir_file
        .distinct_types(true, true)
        .iter()
        .filter_map(|ty| {
            if let IrType::StructRef(_) = ty {
                Some(ty.rust_wire_type())
            } else {
                None
            }
        })
        .collect();

    let temp_dart_wire_file = tempfile::NamedTempFile::new()?;
    let temp_bindgen_c_output_file = tempfile::Builder::new().suffix(".h").tempfile()?;
    with_changed_file(
        config
            .rust_native_output_path()
            .as_deref()
            .and_then(Path::to_str)
            .unwrap(),
        DUMMY_WIRE_CODE_FOR_BINDGEN,
        || {
            commands::bindgen_rust_to_dart(
                &config.rust_crate_dir,
                temp_bindgen_c_output_file
                    .path()
                    .as_os_str()
                    .to_str()
                    .unwrap(),
                temp_dart_wire_file.path().as_os_str().to_str().unwrap(),
                &config.dart_wire_class_name(),
                c_struct_names,
                &config.llvm_path[..],
                &config.llvm_compiler_opts,
            );
        },
    );

    let effective_func_names = [
        generated_rust.extern_func_names,
        EXTRA_EXTERN_FUNC_NAMES.to_vec(),
    ]
    .concat();
    let c_dummy_code = generator::c::generate_dummy(&effective_func_names);
    for output in &config.c_output_path {
        fs::create_dir_all(Path::new(output).parent().unwrap()).unwrap();
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

    sanity_check(&generated_dart_wire.body, &config.dart_wire_class_name());

    let generated_dart_decl_all = generated_dart.decl_code;
    let generated_dart_impl_all = &generated_dart.impl_code + &generated_dart_wire;
    if config.wasm || config.dart_decl_output_path.is_some() {
        // --wasm forces codegen to split declarations,
        // but still allows the user to customize the name of the declaration file.
        let dart_decl_output_path = config.dart_fallback_decl_output_path();
        let impl_import_decl =
            import_decl(dart_decl_output_path.to_str().unwrap(), dart_output_dir);
        fs::write(
            &dart_decl_output_path,
            (&generated_dart.file_prelude + &generated_dart_decl_all).to_text(),
        )?;
        fs::write(
            &config.dart_output_path,
            (&generated_dart.file_prelude + &impl_import_decl + &generated_dart_impl_all).to_text(),
        )?;
        if let Some(wasm) = generated_dart.wasm_code {
            let wasm_path = config.dart_wasm_output_path().unwrap();
            fs::write(
                &wasm_path,
                (&generated_dart.file_prelude + &impl_import_decl + &wasm)
                    .to_text(),
            )?;
        }
    } else {
        fs::write(
            &config.dart_output_path,
            (&generated_dart.file_prelude + &generated_dart_decl_all + &generated_dart_impl_all)
                .to_text(),
        )?;
    }

    let dart_root = &config.dart_root;
    if needs_freezed && config.build_runner {
        let dart_root = dart_root.as_ref().unwrap_or_else(|| {
            panic!(
                "build_runner configured to run, but Dart root could not be inferred.
Please specify --dart-root, or disable build_runner with --no-build-runner."
            )
        });
        commands::build_runner(dart_root);
        commands::format_dart(
            config.dart_output_freezed_path().to_str().unwrap(),
            config.dart_format_line_length,
        );
    }

    commands::format_dart(&config.dart_output_path, config.dart_format_line_length);
    if config.wasm || config.dart_decl_output_path.is_some() {
        commands::format_dart(
            config
                .dart_fallback_decl_output_path()
                .as_path()
                .to_str()
                .unwrap(),
            config.dart_format_line_length,
        );
    }
    if config.wasm {
        commands::format_dart(
            &config.dart_wasm_output_path().unwrap(),
            config.dart_format_line_length,
        );
    }

    info!("Success! Now go and use it :)");

    Ok(())
}

fn import_decl(dart_decl_output_path: &str, dart_output_dir: &Path) -> DartBasicCode {
    DartBasicCode {
        import: format!(
            "import \"{}\";",
            diff_paths(dart_decl_output_path, dart_output_dir)
                .unwrap()
                .to_str()
                .unwrap()
        ),
        ..Default::default()
    }
}
