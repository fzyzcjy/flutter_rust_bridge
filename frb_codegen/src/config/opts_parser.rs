use crate::config::opts::Opts;
use crate::config::raw_opts::RawOpts;
use crate::config::refine_c_output::get_refined_c_output;
use crate::utils::misc::{is_same_directory, BlockIndex, ExtraTraitForVec, PathExt};
use anyhow::*;
use clap::CommandFactory;
use convert_case::{Case, Casing};
use itertools::Itertools;
use std::borrow::Cow;

use std::ffi::OsString;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{env, fs};
use toml::Value;

use super::all_configs::AllConfigs;

// the default shared file/type name, for both the generated Rust, Dart and C header file
const BRIDGE_GENERATED_SHARED: &str = "bridge_generated_shared";

/// Parses the raw command-line options,
/// and returns a tuple containing a vector of `Opts` structs
/// and a vector of unique symbol name(s)/Api(s).
/// Both the 2 are essential for code generation.
pub fn parse_configs_and_symbols(mut raw: RawOpts) -> Result<(AllConfigs, Vec<String>)> {
    if let Some(config) = raw.config_file {
        raw = parse_yaml(&config);
    }

    use clap::error::ErrorKind;
    // rust input path(s)
    log::debug!("the raw rust_input_paths are:{:?}", raw.rust_input); // TODO: delete
    let rust_input_paths = get_valid_canon_paths(&raw.rust_input);
    assert!(!rust_input_paths.is_empty());
    log::debug!("the rust_input_paths are:{:?}", rust_input_paths); // TODO: delete
    for rust_input in rust_input_paths.iter() {
        if rust_input.contains("lib.rs") {
            log::warn!("Do not use `lib.rs` as a Rust input. Please put code to be generated in an `api.rs` or similar.");
            break;
        }
    }

    // dart output path(s)
    let dart_output_paths = get_valid_canon_paths(&raw.dart_output);
    if dart_output_paths.len() != rust_input_paths.len() {
        raw_opts_bail(
            ErrorKind::WrongNumberOfValues,
            "--dart-output's inputs should match --rust-input's length".into(),
        )
    }

    // rust crate dir(s)
    let rust_crate_dirs = raw.rust_crate_dir.unwrap_or_else(|| {
        rust_input_paths
            .iter()
            .map(|each_rust_input_path| {
                fallback_rust_crate_dir(each_rust_input_path)
                    .unwrap_or_else(|_| panic!("{}", format_fail_to_guess_error("rust_crate_dir")))
            })
            .collect::<Vec<_>>()
    });
    let rust_crate_dirs = rust_crate_dirs
        .iter()
        .map(|each_path| canon_path(each_path))
        .collect::<Vec<_>>();
    if rust_crate_dirs.len() != rust_input_paths.len() {
        raw_opts_bail(
            ErrorKind::WrongNumberOfValues,
            "--rust-crate-dir's inputs should match --rust-input's length".into(),
        );
    }

    // manifest path(s)
    let manifest_paths = rust_crate_dirs
        .iter()
        .map(|each| {
            let mut path = PathBuf::from_str(each).unwrap();
            path.push("Cargo.toml");
            path_to_string(path).unwrap()
        })
        .collect::<Vec<_>>();

    // rust output path(s)
    let rust_output_paths = get_outputs_for_flag_requires_full_data(
        &raw.rust_output,
        &rust_input_paths,
        &fallback_rust_output_path,
        "rust_output",
    );
    let rust_output_paths = rust_output_paths
        .iter()
        .map(|each_path| canon_path(each_path))
        .collect::<Vec<_>>();
    if rust_output_paths.len() != rust_input_paths.len() {
        raw_opts_bail(
            ErrorKind::WrongNumberOfValues,
            "--rust-output's inputs should match --rust-input's length".into(),
        );
    }

    // rust/dart shared output path, used
    let single_block_mode = raw.rust_input.len() == 1;
    let (shared_rust_output_path, shared_dart_output_path) = if single_block_mode {
        // single block case
        (None, None)
    } else {
        // multi-blocks case

        // 1.Check if all regular paths are in the same directory
        if !is_same_directory(&rust_output_paths) || !is_same_directory(&dart_output_paths) {
            panic!("For multi-blocks case, paths in flag `rust-output`/`dart-output` respectively should be in the same directory");
        }
        // 2.Get proper raw shared rust path
        let p = raw
            .rust_output
            .clone()
            .map(|vec| vec.get(0).cloned().unwrap_or_default())
            .unwrap_or_default();
        let raw_path = raw
            .shared_rust_output
            .clone()
            .unwrap_or_else(|| format!("./{BRIDGE_GENERATED_SHARED}.rs"));
        let raw_shared_rust_output_path = raw.shared_rust_output.clone().unwrap_or_else(|| {
            let directory = Path::new(&p).parent().unwrap_or(Path::new(""));
            let shared_rust_file_name = Path::new(&raw_path).get_file_name();
            Path::join(directory, shared_rust_file_name)
                .into_os_string()
                .into_string()
                .unwrap()
        });
        // 3.Return rust/dart output path with full directories
        let shared_rust_output_path =
            get_valid_canon_paths(&[raw_shared_rust_output_path])[0].clone();
        let shared_dart_output_path = Path::join(
            Path::new(&dart_output_paths[0]).parent().unwrap(),
            format!(
                "{}.dart",
                Path::new(&shared_rust_output_path)
                    .file_name_str()
                    .unwrap()
                    .replace(".rs", "")
                    .replace(".dart", "")
            ),
        )
        .into_os_string()
        .into_string()
        .unwrap();

        (Some(shared_rust_output_path), Some(shared_dart_output_path))
    };

    // class name(s)
    let class_names = get_outputs_for_flag_requires_full_data(
        &raw.class_name,
        &rust_crate_dirs,
        &fallback_class_name,
        "class_name",
    );
    if !class_names.find_duplicates_in_order(true).is_empty() {
        raw_opts_bail(
            ErrorKind::ValueValidation,
            "there should be no duplication in --class-name's inputs".into(),
        );
    };
    if class_names.len() != rust_input_paths.len() {
        raw_opts_bail(
            ErrorKind::WrongNumberOfValues,
            "--class-name's inputs should match --rust-input's length".into(),
        );
    }

    let skip_deps_check = raw.skip_deps_check;

    let refined_c_outputs = get_refined_c_output(
        &raw.c_output,
        &shared_rust_output_path,
        &raw.extra_c_output_path,
        &rust_input_paths,
    );

    // dart root(s)
    let dart_roots: Vec<_> = match raw.dart_root {
        Some(dart_roots) => dart_roots
            .into_iter()
            .map(|each_path| Some(canon_path(&each_path)))
            .collect(),
        None => dart_output_paths
            .iter()
            .map(|each_dart_output_path| fallback_dart_root(each_dart_output_path).ok())
            .collect(),
    };

    // build Opt for each rust api block
    let dart_decl_output_path = raw
        .dart_decl_output
        .as_ref()
        .map(|s| canon_path(s.as_str()));
    let dart_format_line_length = raw.dart_format_line_length;
    let dart_enums_style = raw.dart_enums_style;
    let llvm_paths = get_llvm_paths(&raw.llvm_path);
    let llvm_compiler_opts = raw.llvm_compiler_opts.clone().unwrap_or_default();
    let skip_add_mod_to_lib = raw.skip_add_mod_to_lib;
    let build_runner = !raw.no_build_runner;
    let bridge_in_method = !raw.no_use_bridge_in_method;
    let wasm = raw.wasm;
    let dart3 = raw.dart3;
    let inline_rust = raw.inline_rust;
    let keep_going = raw.keep_going;
    let extra_headers = raw.extra_headers.unwrap_or({
        if raw.no_use_bridge_in_method {
            "import 'ffi.io.dart' if (dart.library.html) 'ffi.web.dart';".to_owned()
        } else {
            "".to_owned()
        }
    });

    let regular_configs = (0..rust_input_paths.len())
        .map(|i| {
            Opts {
                rust_input_path: rust_input_paths[i].clone(),
                dart_output_path: dart_output_paths[i].clone(),
                dart_decl_output_path: dart_decl_output_path.clone(),
                c_output_paths: refined_c_outputs[i].clone(),
                rust_crate_dir: rust_crate_dirs[i].clone(),
                rust_output_path: rust_output_paths[i].clone(),
                class_name: class_names[i].clone(),
                dart_format_line_length,
                dart_enums_style,
                skip_add_mod_to_lib, //same for all rust api blocks
                llvm_path: llvm_paths.clone(),
                llvm_compiler_opts: llvm_compiler_opts.clone(),
                manifest_path: manifest_paths[i].clone(),
                dart_root: dart_roots[i].clone(),
                build_runner, //same for all rust api blocks
                block_index: BlockIndex(Some(i)),
                skip_deps_check,
                wasm_enabled: wasm,
                dart3,
                inline_rust,
                shared: false,
                bridge_in_method,
                keep_going,
                extra_headers: extra_headers.clone(),
            }
        })
        .collect::<Vec<_>>();

    // if shared API-block(config) is essential, generate and add it to vec
    assert!(!regular_configs.is_empty());
    let all_configs = if regular_configs.len() == 1 {
        regular_configs
    } else {
        // NOTE: since there would be at least 1 shared API called `free_WireSyncReturn` for multi-blocks,
        // the extra shared config is essential.
        let shared_config = Opts {
            rust_input_path: "".into(), // this field is meangingless for shared block, so it's empty.
            rust_output_path: shared_rust_output_path.unwrap(),
            dart_output_path: shared_dart_output_path.clone().unwrap(),
            dart_decl_output_path,
            rust_crate_dir: regular_configs[0].rust_crate_dir.clone(),
            c_output_paths: refined_c_outputs.last().unwrap().clone(),
            class_name: Path::new(&shared_dart_output_path.unwrap())
                .file_name_str()
                .unwrap()
                .replace(".rs", "")
                .replace(".dart", "")
                .to_case(Case::Pascal),
            dart_format_line_length,
            skip_add_mod_to_lib,
            llvm_path: llvm_paths,
            llvm_compiler_opts,
            manifest_path: manifest_paths[0].clone(),
            dart_root: dart_roots[0].clone(),
            build_runner,
            block_index: BlockIndex(None),
            skip_deps_check: true,
            wasm_enabled: wasm,
            inline_rust,
            shared: true,
            dart_enums_style,
            bridge_in_method, //TODO: check for shared Opt
            extra_headers,    //TODO: check for shared Opt
            dart3,
            keep_going,
        };
        [regular_configs, vec![shared_config]].concat()
    };

    let all_configs = AllConfigs::new(all_configs);
    let all_symbols = all_configs.get_all_symbols();
    Ok((all_configs, all_symbols))
}

#[inline(never)]
pub(crate) fn raw_opts_bail(err: clap::error::ErrorKind, message: Cow<str>) -> ! {
    RawOpts::command().error(err, message).exit()
}

fn get_outputs_for_flag_requires_full_data(
    strings: &Option<Vec<String>>,
    fallback_paths: &[String],
    fallback_func: &dyn Fn(&str) -> Result<String>,
    field_str: &str, // str with underline, like "class_name"
) -> Vec<String> {
    strings.clone().unwrap_or_else(|| -> Vec<String> {
        if fallback_paths.len() == 1 {
            vec![fallback_func(&fallback_paths[0])
                .unwrap_or_else(|_| panic!("{}", format_fail_to_guess_error(field_str)))]
        } else {
            let strs = field_str.split('_').collect::<Vec<_>>();
            let raw_str = strs.join(" ");
            let flag_str = strs.join("-");
            raw_opts_bail(
                clap::error::ErrorKind::ValueValidation,
                format!("for more than 1 rust blocks, please specify each {raw_str} clearly with flag \"{flag_str}\"").into()
            )
        }
    })
}

fn get_llvm_paths(llvm_path: &Option<Vec<String>>) -> Vec<String> {
    llvm_path.clone().unwrap_or_else(|| {
        vec![
            "/opt/homebrew/opt/llvm".to_owned(), // Homebrew root
            "/usr/local/opt/llvm".to_owned(),    // Homebrew x86-64 root
            // Possible Linux LLVM roots
            "/usr/lib/llvm-9".to_owned(),
            "/usr/lib/llvm-10".to_owned(),
            "/usr/lib/llvm-11".to_owned(),
            "/usr/lib/llvm-12".to_owned(),
            "/usr/lib/llvm-13".to_owned(),
            "/usr/lib/llvm-14".to_owned(),
            "/usr/lib/".to_owned(),
            "/usr/lib64/".to_owned(),
            "C:/Program Files/llvm".to_owned(), // Default on Windows
            "C:/msys64/mingw64".to_owned(), // https://packages.msys2.org/package/mingw-w64-x86_64-clang
        ]
    })
}

/// Terminates the program if either the file doesn't exist, or is invalid.
fn parse_yaml(path: &str) -> RawOpts {
    use clap::error::ErrorKind;

    let file = File::open(path)
        .map_err(|err| {
            raw_opts_bail(
                ErrorKind::Io,
                format!("Could not open {path}: {err}").into(),
            )
        })
        .unwrap();
    let config = serde_yaml::from_reader(file)
        .map_err(|err| {
            raw_opts_bail(
                ErrorKind::InvalidValue,
                format!("Could not parse config from {path}: {err}").into(),
            )
        })
        .unwrap();
    anchor_config(config, path)
}

fn anchor_config(config: RawOpts, config_path: &str) -> RawOpts {
    let config_path = canon_pathbuf(config_path);
    let cwd = config_path.parent().unwrap();
    let anchor = |path: String| {
        if Path::new(&path).is_absolute() {
            path
        } else {
            cwd.join(&path).to_str().unwrap().to_owned()
        }
    };
    let anchor_many = |paths: Vec<String>| paths.into_iter().map(anchor).collect_vec();

    // Don't collapse reassignments into a spread here, as future configs may need to be
    // correctly re-anchored.
    RawOpts {
        rust_input: anchor_many(config.rust_input),
        dart_output: anchor_many(config.dart_output),
        rust_output: config.rust_output.map(anchor_many),
        dart_decl_output: config.dart_decl_output.map(anchor),
        c_output: config.c_output.map(anchor_many),
        extra_c_output_path: config.extra_c_output_path.map(anchor_many),
        rust_crate_dir: config.rust_crate_dir.map(anchor_many),
        dart_root: config.dart_root.map(anchor_many),
        config_file: config.config_file,
        class_name: config.class_name,
        dart_format_line_length: config.dart_format_line_length,
        dart_enums_style: config.dart_enums_style,
        skip_add_mod_to_lib: config.skip_add_mod_to_lib,
        llvm_path: config.llvm_path,
        llvm_compiler_opts: config.llvm_compiler_opts,
        no_build_runner: config.no_build_runner,
        verbose: config.verbose,
        wasm: config.wasm,
        dart3: config.dart3,
        inline_rust: config.inline_rust,
        skip_deps_check: config.skip_deps_check,
        no_use_bridge_in_method: config.no_use_bridge_in_method,
        extra_headers: config.extra_headers,
        shared_rust_output: config.shared_rust_output,
        keep_going: config.keep_going,
        #[cfg(feature = "serde")]
        dump: config.dump,
    }
}

fn get_valid_canon_paths(paths: &[String]) -> Vec<String> {
    paths
        .iter()
        .filter(|p| !p.trim().is_empty())
        .map(|p| canon_path(p))
        .collect::<Vec<_>>()
}

pub(crate) fn format_fail_to_guess_error(name: &str) -> String {
    format!("fail to guess {name}, please specify it manually in command line arguments")
}

fn fallback_rust_crate_dir(rust_input_path: &str) -> Result<String> {
    let mut dir_curr = Path::new(rust_input_path)
        .parent()
        .context("Unexpected value for rust-crate-dir")?;

    loop {
        let path_cargo_toml = dir_curr.join("Cargo.toml");

        if path_cargo_toml.exists() {
            return Ok(dir_curr
                .as_os_str()
                .to_str()
                .context("Not a UTF-8 path")?
                .to_string());
        }

        if let Some(next_parent) = dir_curr.parent() {
            dir_curr = next_parent;
        } else {
            break;
        }
    }
    Err(anyhow!(
        "look at parent directories but none contains Cargo.toml"
    ))
}

fn fallback_rust_output_path(rust_input_path: &str) -> Result<String> {
    Ok(Path::new(rust_input_path)
        .with_file_name("bridge_generated.rs")
        .to_str()
        .context("Not a UTF-8 path")?
        .to_string())
}

fn fallback_dart_root(dart_output_path: &str) -> Result<String> {
    let mut res = canon_pathbuf(dart_output_path);
    while res.pop() {
        if res.join("pubspec.yaml").is_file() {
            return res
                .to_str()
                .map(ToString::to_string)
                .context("Not a UTF-8 path");
        }
    }
    Err(anyhow!(
        "Root of Dart library could not be inferred from Dart output"
    ))
}

fn fallback_class_name(rust_crate_dir: &str) -> Result<String> {
    let cargo_toml_path = Path::new(rust_crate_dir).join("Cargo.toml");
    let cargo_toml_content = fs::read_to_string(cargo_toml_path)?;

    let cargo_toml_value = cargo_toml_content.parse::<Value>()?;
    let package_name = cargo_toml_value
        .get("package")
        .context("no `package` in Cargo.toml")?
        .get("name")
        .context("no `name` in Cargo.toml")?
        .as_str()
        .unwrap();

    Ok(package_name.to_case(Case::Pascal))
}

pub(crate) fn canon_path(sub_path: &str) -> String {
    let path = canon_pathbuf(sub_path);
    path_to_string(path).unwrap_or_else(|_| panic!("fail to parse path: {}", sub_path))
}

fn canon_pathbuf(sub_path: &str) -> PathBuf {
    let mut path =
        env::current_dir().unwrap_or_else(|_| panic!("fail to parse path: {}", sub_path));
    path.push(sub_path);
    path
}

fn path_to_string(path: PathBuf) -> Result<String, OsString> {
    path.into_os_string().into_string()
}
