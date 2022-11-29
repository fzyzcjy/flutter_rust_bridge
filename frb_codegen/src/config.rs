use std::borrow::Cow;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use clap::IntoApp;
use clap::Parser;
use convert_case::{Case, Casing};
use serde::Deserialize;
use toml::Value;

use crate::ir::IrFile;
use crate::parser;
use crate::utils::BlockIndex;

#[derive(Parser, Debug, PartialEq, Eq, Deserialize, Default)]
#[clap(version, setting(clap::AppSettings::DeriveDisplayOrder))]
pub struct RawOpts {
    /// Path of input Rust code
    #[clap(short, long, required = true, multiple_values = true)]
    pub rust_input: Vec<String>,
    /// Path of output generated Dart code
    #[clap(short, long, required = true, multiple_values = true)]
    pub dart_output: Vec<String>,
    /// If provided, generated Dart declaration code to this separate file
    #[clap(long)]
    pub dart_decl_output: Option<String>,
    /// Output path of generated C header
    #[clap(short, long)]
    pub c_output: Option<Vec<String>>,
    /// Crate directory for your Rust project
    #[clap(long, multiple_values = true)]
    pub rust_crate_dir: Option<Vec<String>>,
    /// Output path of generated Rust code
    #[clap(long, multiple_values = true)]
    pub rust_output: Option<Vec<String>>,
    /// Generated class name
    #[clap(long, multiple_values = true)]
    pub class_name: Option<Vec<String>>,
    /// Line length for Dart formatting
    #[clap(long, default_value = "80")]
    pub dart_format_line_length: u32,
    /// Skip automatically adding `mod bridge_generated;` to `lib.rs`
    #[clap(long)]
    pub skip_add_mod_to_lib: bool,
    /// Path to the installed LLVM
    #[clap(long, multiple_values = true)]
    pub llvm_path: Option<Vec<String>>,
    /// LLVM compiler opts
    #[clap(long)]
    pub llvm_compiler_opts: Option<String>,
    /// Path to root of Dart project, otherwise inferred from --dart-output
    #[clap(long, multiple_values = true)]
    pub dart_root: Option<Vec<String>>,
    /// Skip running build_runner even when codegen-required code is detected
    #[clap(long)]
    pub no_build_runner: bool,
    /// Show debug messages.
    #[clap(short, long)]
    pub verbose: bool,
    /// Enable WASM module generation.
    /// Requires: --dart-decl-output
    #[clap(long)]
    pub wasm: bool,
    /// Inline declaration of Rust bridge modules
    #[clap(long)]
    pub inline_rust: bool,
    /// Skip dependencies check.
    #[clap(long)]
    pub skip_deps_check: bool,
}

#[derive(Debug, Clone)]
pub struct Opts {
    pub rust_input_path: String,
    pub dart_output_path: String,
    pub dart_decl_output_path: Option<String>,
    pub c_output_path: Vec<String>,
    pub rust_crate_dir: String,
    pub rust_output_path: String,
    pub class_name: String,
    pub dart_format_line_length: u32,
    pub skip_add_mod_to_lib: bool,
    pub llvm_path: Vec<String>,
    pub llvm_compiler_opts: String,
    pub manifest_path: String,
    pub dart_root: Option<String>,
    pub build_runner: bool,
    pub block_index: BlockIndex,
    pub skip_deps_check: bool,
    pub wasm_enabled: bool,
    pub inline_rust: bool,
    pub idx: usize,
    pub count: usize,
}

pub fn parse(raw: RawOpts) -> Vec<Opts> {
    fn bail(err: clap::ErrorKind, message: Cow<str>) {
        RawOpts::command().error(err, message).exit()
    }
    // rust input path(s)
    let rust_input_paths = get_valid_canon_paths(&raw.rust_input);

    // dart output path(s)
    let dart_output_paths = get_valid_canon_paths(&raw.dart_output);
    if dart_output_paths.len() != rust_input_paths.len() {
        bail(
            clap::ErrorKind::WrongNumberOfValues,
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
        bail(
            clap::ErrorKind::WrongNumberOfValues,
            "--rust-crate-dir's inputs should match --rust-input's length".into(),
        );
    }

    // manifest path(s)
    let manifest_paths = rust_crate_dirs
        .iter()
        .map(|each| {
            let mut path = std::path::PathBuf::from_str(each).unwrap();
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
        bail(
            clap::ErrorKind::WrongNumberOfValues,
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
    if class_names.len() != rust_input_paths.len() {
        bail(
            clap::ErrorKind::WrongNumberOfValues,
            "--class-name's inputs should match --rust-input's length".into(),
        );
    }

    let skip_deps_check = raw.skip_deps_check;

    // c output path(s) (only 1 list is needed, nothing to do with number of rust_input_paths)
    let c_output_paths = raw
        .c_output
        .map(|outputs| {
            outputs
                .iter()
                .map(|output| canon_path(output))
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| {
            vec![fallback_c_output_path()
                .unwrap_or_else(|_| panic!("{}", format_fail_to_guess_error("c_output")))]
        });

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
    let llvm_paths = get_llvm_paths(&raw.llvm_path);
    let llvm_compiler_opts = raw.llvm_compiler_opts.clone().unwrap_or_default();
    let skip_add_mod_to_lib = raw.skip_add_mod_to_lib;
    let build_runner = !raw.no_build_runner;
    let wasm = raw.wasm;
    let inline_rust = raw.inline_rust;

    (0..rust_input_paths.len())
        .map(|i| {
            Opts {
                rust_input_path: rust_input_paths[i].clone(),
                dart_output_path: dart_output_paths[i].clone(),
                dart_decl_output_path: dart_decl_output_path.clone(),
                c_output_path: c_output_paths.clone(), //same for all rust api blocks
                rust_crate_dir: rust_crate_dirs[i].clone(),
                rust_output_path: rust_output_paths[i].clone(),
                class_name: class_names[i].clone(),
                dart_format_line_length,
                skip_add_mod_to_lib, //same for all rust api blocks
                llvm_path: llvm_paths.clone(),
                llvm_compiler_opts: llvm_compiler_opts.clone(),
                manifest_path: manifest_paths[i].clone(),
                dart_root: dart_roots[i].clone(),
                build_runner, //same for all rust api blocks
                block_index: BlockIndex(i),
                skip_deps_check,
                wasm_enabled: wasm,
                inline_rust,
                idx: i,
                count: rust_input_paths.len(),
            }
        })
        .collect()
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
            RawOpts::command()
                .error(
                    clap::ErrorKind::ValueValidation,
                    format!(
                        "for more than 1 rust blocks, please specify each {} clearly with flag \"{}\"",
                        raw_str, flag_str
                    ),
                )
                .exit()
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

fn get_valid_canon_paths(paths: &[String]) -> Vec<String> {
    paths
        .iter()
        .filter(|p| !p.trim().is_empty())
        .map(|p| canon_path(p))
        .collect::<Vec<_>>()
}

fn format_fail_to_guess_error(name: &str) -> String {
    format!(
        "fail to guess {}, please specify it manually in command line arguments",
        name
    )
}

fn fallback_rust_crate_dir(rust_input_path: &str) -> Result<String> {
    let mut dir_curr = Path::new(rust_input_path)
        .parent()
        .ok_or_else(|| anyhow!(""))?;

    loop {
        let path_cargo_toml = dir_curr.join("Cargo.toml");

        if path_cargo_toml.exists() {
            return Ok(dir_curr
                .as_os_str()
                .to_str()
                .ok_or_else(|| anyhow!(""))?
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

fn fallback_c_output_path() -> Result<String> {
    let named_temp_file = Box::leak(Box::new(tempfile::Builder::new().suffix(".h").tempfile()?));
    Ok(named_temp_file
        .path()
        .to_str()
        .ok_or_else(|| anyhow!(""))?
        .to_string())
}

fn fallback_rust_output_path(rust_input_path: &str) -> Result<String> {
    Ok(Path::new(rust_input_path)
        .parent()
        .ok_or_else(|| anyhow!(""))?
        .join("bridge_generated.rs")
        .to_str()
        .ok_or_else(|| anyhow!(""))?
        .to_string())
}

fn fallback_dart_root(dart_output_path: &str) -> Result<String> {
    let mut res = canon_pathbuf(dart_output_path);
    while res.pop() {
        if res.join("pubspec.yaml").is_file() {
            return res
                .to_str()
                .map(ToString::to_string)
                .ok_or_else(|| anyhow!("Non-utf8 path"));
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
        .ok_or_else(|| anyhow!("no `package` in Cargo.toml"))?
        .get("name")
        .ok_or_else(|| anyhow!("no `name` in Cargo.toml"))?
        .as_str()
        .ok_or_else(|| anyhow!(""))?;

    Ok(package_name.to_case(Case::Pascal))
}

fn canon_path(sub_path: &str) -> String {
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

impl Opts {
    pub fn get_ir_file(&self) -> IrFile {
        // info!("Phase: Parse source code to AST");
        let source_rust_content = fs::read_to_string(&self.rust_input_path)
            .unwrap_or_else(|_| panic!("panic with file: {}", &self.rust_input_path));
        let file_ast = syn::parse_file(&source_rust_content).unwrap();

        // info!("Phase: Parse AST to IR");

        parser::parse(&source_rust_content, file_ast, &self.manifest_path)
    }

    pub fn dart_api_class_name(&self) -> String {
        self.class_name.clone()
    }

    pub fn dart_api_impl_class_name(&self) -> String {
        format!("{}Impl", self.class_name)
    }

    pub fn dart_wire_class_name(&self) -> String {
        format!("{}Wire", self.class_name)
    }

    pub fn dart_platform_class_name(&self) -> String {
        format!("{}Platform", self.class_name)
    }

    pub fn dart_wasm_module(&self) -> String {
        format!("{}WasmModule", self.class_name)
    }

    pub(crate) fn dart_output_root(&self) -> Option<&str> {
        Path::new(
            self.dart_decl_output_path
                .as_ref()
                .unwrap_or(&self.dart_output_path),
        )
        .file_stem()?
        .to_str()
    }

    pub fn dart_wasm_output_path(&self) -> PathBuf {
        Path::new(&self.dart_output_path).with_extension("web.dart")
    }

    pub fn dart_io_output_path(&self) -> PathBuf {
        Path::new(&self.dart_output_path).with_extension("io.dart")
    }

    pub fn dart_common_output_path(&self) -> PathBuf {
        Path::new(&self.dart_output_path).with_extension("common.dart")
    }

    pub fn rust_wasm_output_path(&self) -> PathBuf {
        Path::new(&self.rust_output_path).with_extension("web.rs")
    }

    pub fn rust_io_output_path(&self) -> PathBuf {
        Path::new(&self.rust_output_path).with_extension("io.rs")
    }

    pub fn dart_root_or_default(&self) -> String {
        self.dart_root
            .clone()
            .unwrap_or_else(|| env!("CARGO_MANIFEST_DIR").to_string())
    }

    pub fn dart_freezed_path(&self) -> PathBuf {
        PathBuf::from(
            self.dart_decl_output_path
                .as_deref()
                .unwrap_or(&self.dart_output_path),
        )
        .with_extension("freezed.dart")
    }
}
