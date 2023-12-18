use crate::commands::BindgenRustToDartArg;

use crate::config::all_configs::AllConfigs;
use crate::ir::IrTypeTrait;
use crate::others::{
    extract_dart_wire_content, modify_dart_wire_content, sanity_check, DartBasicCode,
    DUMMY_WIRE_CODE_FOR_BINDGEN, EXTRA_EXTERN_FUNC_NAMES,
};
use crate::utils::misc::{with_changed_file, ExtraTraitForVec};
use crate::{command_run, commands, ensure_tools_available, generator, Opts};
use convert_case::{Case, Casing};
use itertools::Itertools;
use log::info;
use pathdiff::diff_paths;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

pub(crate) fn generate_dart_code(
    config: &Opts,
    all_configs: &AllConfigs,
    generated_rust: generator::rust::Output,
    all_symbols: &[String],
) -> crate::Result {
    info!("Phase: Generating Dart bindings for Rust");

    let dart_root = config.dart_root_or_default();
    ensure_tools_available(&dart_root, config.skip_deps_check)?;

    // phase-step1: generate (temporary) c file(s)
    let temp_dart_wire_file = tempfile::NamedTempFile::new()?;
    let temp_bindgen_c_output_file = tempfile::Builder::new().suffix(".h").tempfile()?;
    // TODO: does `get_exclude_symbols` needed since Struct `AllConfig` introduced?
    let mut exclude_symbols = generated_rust.get_exclude_symbols(all_symbols, config, all_configs);
    let mut extra_forward_declarations = Vec::new();
    // extra refinement for multi-blocks case
    if config.shared {
        // refine `exclude_symbols`
        for c in all_configs.get_regular_configs().iter() {
            let (wire_types, wire_funcs) = get_wire_types_funcs_for_c_file(c, all_configs);
            exclude_symbols.extend(wire_types);
            exclude_symbols.extend(wire_funcs);
        }
    } else if all_configs.is_multi_blocks_case() {
        // refine `exclude_symbols`
        for regular_config in all_configs.get_regular_configs().iter() {
            if regular_config.block_index == regular_config.block_index {
                continue;
            }
            let (wire_types, wire_funcs) =
                get_wire_types_funcs_for_c_file(regular_config, all_configs);
            exclude_symbols.extend(wire_types);
            exclude_symbols.extend(wire_funcs);
        }
        // refine `extra_forward_declarations`
        let types_used_in_the_block =
            &all_configs.get_types(config.block_index, false, true, true, true);
        let types_only_used_in_the_block =
            &all_configs.get_types(config.block_index, true, true, true, true);
        let shared_type_in_the_block = types_used_in_the_block
            .iter()
            .filter(|each| !types_only_used_in_the_block.contains(each))
            .collect::<Vec<_>>();

        let shared_type_names = shared_type_in_the_block
            .iter()
            .filter(|each| each.is_struct_ref_or_enum_ref_or_record() || each.is_list(false))
            .map(|each| each.get_rust_name())
            .collect::<Vec<_>>();
        let extra_forward_declaration = shared_type_names
            .iter()
            .map(|each| format!("typedef struct wire_{each} wire_{each}"))
            .collect::<Vec<_>>();

        extra_forward_declarations.extend(extra_forward_declaration);
    }
    // parse and the specific c code: `temp_bindgen_c_output_file`
    let c_struct_names = all_configs.get_c_struct_names(config.block_index);
    log::debug!(
        "for block:{}, extra_forward_declarations:\n{:#?}",
        config.block_index,
        extra_forward_declarations,
    ); // TODO: delete
    log::debug!(
        "for block:{}, c_struct_names:\n{:#?}",
        config.block_index,
        c_struct_names,
    ); // TODO: delete
    log::debug!(
        "for block:{}, config.dart_wire_class_name():\n{:#?}",
        config.block_index,
        config.dart_wire_class_name(),
    ); // TODO: delete
    log::debug!(
        "for block:{}, exclude_symbols:\n{:#?}",
        config.block_index,
        exclude_symbols,
    ); // TODO: delete
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
                    c_struct_names,
                    exclude_symbols,
                    extra_forward_declarations,
                    llvm_install_path: &config.llvm_path[..],
                    llvm_compiler_opts: &config.llvm_compiler_opts,
                },
                &dart_root,
            )
            .map_err(Into::into)
        },
    )?;
    // refine the funcs used in dummy function
    let effective_func_names = if !all_configs.is_multi_blocks_case() || config.shared {
        [
            generated_rust.extern_func_names,
            EXTRA_EXTERN_FUNC_NAMES.to_vec(),
        ]
        .concat()
    } else {
        // TODO: check if `EXTRA_EXTERN_FUNC_NAMES` needed for regular one in multi case?
        generated_rust.extern_func_names
    };
    // write all c code into files for each `c_output_path`
    for (i, each_path) in config.c_output_paths.iter().enumerate() {
        let c_dummy_code =
            generator::c::generate_dummy(config, all_configs, &effective_func_names, i);
        fs::create_dir_all(Path::new(each_path).parent().unwrap())?;
        fs::write(
            each_path,
            fs::read_to_string(&temp_bindgen_c_output_file)? + "\n" + &c_dummy_code,
        )?;
    }

    // phase-step2: generate raw dart code instance from the generated c file(s)
    let generated_dart_wire_code_raw = fs::read_to_string(temp_dart_wire_file)?;
    let generated_dart_wire = extract_dart_wire_content(&modify_dart_wire_content(
        &generated_dart_wire_code_raw,
        config,
        all_configs,
    ));
    sanity_check(&generated_dart_wire.body, &config.dart_wire_class_name())?;

    // phase-step3: compose dart codes and write to file
    let generated_dart = all_configs.generate_dart(config, &generated_rust.wasm_exports);
    let generated_dart_decl_all = &generated_dart.decl_code;
    let generated_dart_impl_io_wire = &generated_dart.impl_code.io + &generated_dart_wire;

    let dart_output_paths = config.get_dart_output_paths();
    let dart_output_dir = Path::new(&dart_output_paths.base_path).parent().unwrap();
    fs::create_dir_all(dart_output_dir)?;

    if let Some(dart_decl_output_path) = &config.dart_decl_output_path {
        write_dart_decls(
            config,
            all_configs,
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

fn get_wire_types_funcs_for_c_file(
    config: &Opts,
    all_configs: &AllConfigs,
) -> (Vec<String>, Vec<String>) {
    let types = all_configs.get_types(config.block_index, true, true, true, true);

    // 1. wire_types
    let wire_types = types
        .iter()
        .map(|each| {
            if each.is_list(true) {
                format!("wire_{}", each.safe_ident())
            } else {
                format!("wire_{}", each.safe_ident().to_case(Case::Pascal))
            }
        })
        .collect::<Vec<_>>();

    // 2. wire_funcs
    let funcs = all_configs
        .get_funcs(config.block_index, true)
        .into_iter()
        .collect::<Vec<_>>();
    let wire_funcs = funcs
        .iter()
        .map(|each| format!("wire_{}", each.name))
        .collect::<Vec<_>>();

    (wire_types, wire_funcs)
}

use std::cell::RefCell;
thread_local!(static HAS_GENERATED_DART_DECL_FILE: RefCell<bool> = RefCell::new(false));

fn write_dart_decls(
    config: &Opts,
    all_configs: &AllConfigs,
    dart_decl_output_path: &str,
    dart_output_dir: &Path,
    generated_dart: &crate::generator::dart::Output,
    generated_dart_decl_all: &DartBasicCode,
    generated_dart_impl_io_wire: &DartBasicCode,
) -> crate::Result {
    // Be it single or multi block(s) case,and whatever the flag `dart-decl-output` is set or not within frb command,
    // just remove the dart declaration file at very first when generating files.
    HAS_GENERATED_DART_DECL_FILE.with(|data| {
        let mut flag = data.borrow_mut();
        if !(*flag) {
            std::fs::remove_file(dart_decl_output_path).unwrap_or_default();
            *flag = true;
        }
    });

    // get essential import content
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

    // if file of `dart_decl_output_path` existed, append content but not create
    let mut file = fs::OpenOptions::new()
        .append(true) // essential for multi-blocks case
        .create(true)
        .open(dart_decl_output_path)
        .unwrap_or_else(|_| panic!("{}", "can't open path\"{dart_decl_output_path}\""));
    std::io::Write::write_all(
        &mut file,
        (&generated_dart.file_prelude + &common_import + generated_dart_decl_all)
            .to_text()
            .as_bytes(),
    )?;

    // erase duplicated lines for multi-blocks case, like the redundant import statements and class definitions
    // NOTE: since dart file with syntax error would make the whole generation stuck,
    // the refinement MUST be done at the end after EACH dart block generation.
    if all_configs.is_multi_blocks_case() {
        let mut contents =
            std::fs::read_to_string(dart_decl_output_path).expect("Unable to read file");
        remove_dupilicated_prehead_and_imports(&mut contents);
        fs::write(dart_decl_output_path, contents)?;
    }

    // refine import for other dart files
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

fn remove_dupilicated_prehead_and_imports(contents: &mut String) {
    let mut lines = contents.lines().collect::<Vec<_>>();
    let mut correct_imports: Vec<String> = Vec::new();
    let mut lines_to_delete = Vec::new();
    let mut check_import_surfix_next_time = false;
    for (i, line) in lines.iter().enumerate() {
        let trimmed_line = line.trim();
        if !check_import_surfix_next_time {
            if trimmed_line.starts_with("part ") || trimmed_line.starts_with("// ") {
                if !correct_imports.contains(&String::from(trimmed_line)) {
                    correct_imports.push(trimmed_line.into());
                }
                lines_to_delete.push(i);
            } else if trimmed_line.starts_with("import ") {
                if trimmed_line.ends_with(';') {
                    if !correct_imports.contains(&String::from(trimmed_line)) {
                        correct_imports.push(trimmed_line.into());
                    }
                } else {
                    check_import_surfix_next_time = true;
                }
                lines_to_delete.push(i);
            }
        } else {
            // check current line with previous `import` line together
            let j = lines_to_delete.last().unwrap();
            let new_import_line = format!("{} {}", lines[*j].trim(), trimmed_line);

            check_import_surfix_next_time = !trimmed_line.ends_with(';');

            if !correct_imports.contains(&new_import_line) {
                correct_imports.push(new_import_line);
                if !check_import_surfix_next_time {
                    correct_imports = correct_imports.find_uniques_in_order(false);
                }
            }
            lines_to_delete.push(i);
        }
    }

    // Sort the indices in descending to prevent issues with subsequent removals
    lines_to_delete.sort_by(|a, b| b.cmp(a));
    for i in lines_to_delete {
        lines.remove(i);
    }

    // add compact import at file top place
    correct_imports.sort();
    // this sort is essential for putting `part ...` after `import ...`
    let imports_lines = correct_imports.join("\n");
    *contents = lines.join("\n");
    *contents = format!("{}\n{}", imports_lines, contents);
}
