use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::Path;

use anyhow::{anyhow, Result};
use convert_case::{Case, Casing};
use serde::Deserialize;
use structopt::clap::AppSettings;
use structopt::StructOpt;
use toml::Value;

#[derive(StructOpt, Debug, PartialEq, Deserialize)]
#[structopt(setting(AppSettings::DeriveDisplayOrder))]
pub struct RawOpts {
    /// Path of input Rust code
    #[structopt(short, long)]
    pub rust_input: String,
    /// Path of output generated Dart code
    #[structopt(short, long)]
    pub dart_output: String,
    /// If provided, generated Dart declaration code to this separate file.
    /// If `--wasm` is specified, this flag has no effect.
    #[structopt(long)]
    pub dart_decl_output: Option<String>,

    /// Path of output generated C header
    #[structopt(short, long)]
    pub c_output: Option<String>,
    /// Crate directory for your Rust project
    #[structopt(long)]
    pub rust_crate_dir: Option<String>,
    /// Path of output generated Rust code
    #[structopt(long)]
    pub rust_output: Option<String>,
    /// Generated class name
    #[structopt(long)]
    pub class_name: Option<String>,
    /// Line length for dart formatting
    #[structopt(long)]
    pub dart_format_line_length: Option<i32>,
    /// Skip automatically adding `mod bridge_generated;` to `lib.rs`
    #[structopt(long)]
    pub skip_add_mod_to_lib: bool,
    /// Path to the installed LLVM
    #[structopt(long)]
    pub llvm_path: Option<String>,
    /// LLVM compiler opts
    #[structopt(long)]
    pub llvm_compiler_opts: Option<String>,
    /// Emit glue code to interface with wasm-bindgen.
    /// Enabling this splits the generated code into their respective modules.
    /// For example, if the output is "foo.rs", a native "foo_native.rs" and
    /// a WASM "foo_web.rs" frontend will be created.
    /// Overrides `--dart-decl-output`.
    #[structopt(long)]
    pub wasm: bool,
}

#[derive(Debug)]
pub struct Opts {
    pub rust_input_path: String,
    pub dart_output_path: String,
    pub dart_decl_output_path: Option<String>,
    pub c_output_path: String,
    pub rust_crate_dir: String,
    pub rust_output_path: String,
    pub class_name: String,
    pub dart_format_line_length: i32,
    pub skip_add_mod_to_lib: bool,
    pub llvm_path: String,
    pub llvm_compiler_opts: String,
    pub wasm: bool,
}

pub fn parse(raw: RawOpts) -> Opts {
    let rust_input_path = canon_path(&raw.rust_input);

    let rust_crate_dir = canon_path(&raw.rust_crate_dir.unwrap_or_else(|| {
        fallback_rust_crate_dir(&rust_input_path)
            .unwrap_or_else(|_| panic!("{}", format_fail_to_guess_error("rust_crate_dir")))
    }));
    let rust_output_path = canon_path(&raw.rust_output.unwrap_or_else(|| {
        fallback_rust_output_path(&rust_input_path)
            .unwrap_or_else(|_| panic!("{}", format_fail_to_guess_error("rust_output")))
    }));
    let class_name = raw.class_name.unwrap_or_else(|| {
        fallback_class_name(&*rust_crate_dir)
            .unwrap_or_else(|_| panic!("{}", format_fail_to_guess_error("class_name")))
    });
    let c_output_path = canon_path(&raw.c_output.unwrap_or_else(|| {
        fallback_c_output_path()
            .unwrap_or_else(|_| panic!("{}", format_fail_to_guess_error("c_output")))
    }));
    let dart_output_path = canon_path(&raw.dart_output);

    Opts {
        rust_input_path,
        dart_output_path,
        dart_decl_output_path: raw
            .dart_decl_output
            .as_ref()
            .map(|s| canon_path(s.as_str())),
        c_output_path,
        rust_crate_dir,
        rust_output_path,
        class_name,
        dart_format_line_length: raw.dart_format_line_length.unwrap_or(80),
        skip_add_mod_to_lib: raw.skip_add_mod_to_lib,
        llvm_path: raw.llvm_path.unwrap_or_else(|| "".to_string()),
        llvm_compiler_opts: raw.llvm_compiler_opts.unwrap_or_else(|| "".to_string()),
        wasm: raw.wasm,
    }
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

/// Wraps `path` with `suffix` and `prefix`, e.g. "/foo/bar.rs" becomes
/// "/foo/{prefix}/bar{suffix}.rs".
/// Returns None for non-utf8 paths.
fn wrap_path(path: &str, suffix: &str, prefix: Option<&str>) -> Option<String> {
    use std::path::*;
    let mut buf = PathBuf::from(path);
    let name = buf.file_name()?.to_str()?.to_string();
    let (name, rest) = name.split_once('.')?;
    buf.set_file_name(format!("{}{}.{}", name, suffix, rest));
    if let Some(prefix) = prefix {
        let name = buf.file_name().map(OsString::from);
        buf.pop();
        buf.push(prefix);
        if let Some(name) = name {
            buf.push(name);
        }
    }
    buf.to_str().map(ToString::to_string)
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
    let mut path =
        env::current_dir().unwrap_or_else(|_| panic!("fail to parse path: {}", sub_path));
    path.push(sub_path);
    path.into_os_string()
        .into_string()
        .unwrap_or_else(|_| panic!("fail to parse path: {}", sub_path))
}

impl Opts {
    pub fn dart_api_class_name(&self) -> String {
        self.class_name.clone()
    }

    pub fn dart_api_impl_class_name(&self) -> String {
        format!("{}Impl", self.class_name)
    }

    pub fn dart_wire_class_name(&self) -> String {
        format!("{}Wire", self.class_name)
    }

    pub fn rust_output_file_name(&self) -> Option<&str> {
        file_prefix_stable(Path::new(&self.rust_output_path))
    }

    pub fn dart_wasm_output_path(&self) -> Option<String> {
        wrap_path(&self.dart_output_path, "_web", None)
    }

    pub fn rust_wasm_output_path(&self) -> Option<String> {
        wrap_path(&self.rust_output_path, "_web", self.rust_output_file_name())
    }

    pub fn rust_native_output_path(&self) -> Option<String> {
        wrap_path(&self.rust_output_path, "_native", self.rust_output_file_name())
    }
}

/// Replacement for `Path::file_prefix`[^1] on stable Rust, which as of writing
/// has not been stabilized yet.
/// Returns None for non-utf8 paths.
///
/// [^1]: https://doc.rust-lang.org/std/path/struct.Path.html#method.file_prefix
#[inline]
pub fn file_prefix_stable(path: &Path) -> Option<&str> {
    Some(path.file_name()?.to_str()?.split_once('.')?.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_wrap_path() {
        assert_eq!(
            wrap_path("some/path.rs", "_suffix", Some("parent")),
            Some("some/parent/path_suffix.rs".into())
        );
    }
    #[test]
    fn test_file_prefix_stable() {
        assert_eq!(
            file_prefix_stable(Path::new("foobar.foo.bar")),
            Some("foobar")
        )
    }
}
