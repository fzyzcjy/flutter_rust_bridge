use crate::config::opts::Opts;
use crate::config::raw_opts::RawOpts;
use crate::config::refine_c_output::get_refined_c_output;
use crate::utils::misc::{find_all_duplicates, BlockIndex};
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

pub fn config_parse(mut raw: RawOpts) -> Vec<Opts> {
    // already done in other parts now
    // if let Some(config) = raw.config_file {
    //     raw = parse_yaml(&config);
    // }

    use clap::error::ErrorKind;
    // rust input path(s)
    let rust_input_paths = get_valid_canon_paths(&raw.rust_input);
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
            .collect_vec()
    });
    let rust_crate_dirs = rust_crate_dirs
        .iter()
        .map(|each_path| canon_path(each_path))
        .collect_vec();
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
        .collect_vec();

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
        .collect_vec();
    if rust_output_paths.len() != rust_input_paths.len() {
        raw_opts_bail(
            ErrorKind::WrongNumberOfValues,
            "--rust-output's inputs should match --rust-input's length".into(),
        );
    }

    // class name(s)
    let class_names = get_outputs_for_flag_requires_full_data(
        &raw.class_name,
        &rust_crate_dirs,
        &fallback_class_name,
        "class_name",
    );
    if !find_all_duplicates(&class_names).is_empty() {
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

    // get correct c outputs for all rust inputs
    let refined_c_outputs =
        get_refined_c_output(&raw.c_output, &raw.extra_c_output_path, &rust_input_paths);

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
            let strs = field_str.split('_').collect_vec();
            let raw_str = strs.join(" ");
            let flag_str = strs.join("-");
            raw_opts_bail(
                clap::error::ErrorKind::ValueValidation,
                format!("for more than 1 rust blocks, please specify each {raw_str} clearly with flag \"{flag_str}\"").into()
            )
        }
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
        keep_going: config.keep_going,
        dump: config.dump,
    }
}

fn get_valid_canon_paths(paths: &[String]) -> Vec<String> {
    paths
        .iter()
        .filter(|p| !p.trim().is_empty())
        .map(|p| canon_path(p))
        .collect_vec()
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
