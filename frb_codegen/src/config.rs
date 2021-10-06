use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use convert_case::{Case, Casing};
use serde::Deserialize;
use structopt::StructOpt;
use toml::Value;

#[derive(StructOpt, Debug, PartialEq, Deserialize)]
#[structopt(name = "flutter_rust_bridge")]
pub struct RawOpts {
    /// Path of input Rust code
    #[structopt(short, long)]
    pub rust_input: String,
    /// Path of output generated Dart code
    #[structopt(short, long)]
    pub dart_output: String,

    /// Path of output generated C header
    #[structopt(short, long)]
    pub c_output: Option<String>,
    /// Crate directory for your Rust project
    #[structopt(short, long)]
    pub rust_crate_dir: Option<String>,
    /// Path of output generated Rust code
    #[structopt(short, long)]
    pub rust_output: Option<String>,
    /// Generated class name
    #[structopt(short, long)]
    pub class_name: Option<String>,
    /// Line length for dart formatting
    #[structopt(short, long)]
    pub dart_format_line_length: Option<i32>,
}

pub struct Opts {
    pub rust_input_path: String,
    pub dart_output_path: String,
    pub c_output_path: String,
    pub rust_crate_dir: String,
    pub rust_output_path: String,
    pub class_name: String,
    pub dart_format_line_length: i32,
}

pub fn parse(raw: RawOpts) -> Opts {
    let rust_input_path = canon_path(&raw.rust_input);

    let rust_crate_dir = canon_path(&raw.rust_crate_dir.unwrap_or_else(|| {
        fallback_rust_crate_dir(&rust_input_path).expect("fail to guess rust_crate_dir")
    }));
    let rust_output_path = canon_path(&raw.rust_output.unwrap_or_else(|| {
        fallback_rust_output_path(&rust_input_path).expect("fail to guess rust_output")
    }));

    let class_name = raw.class_name.unwrap_or_else(|| {
        fallback_class_name(&*rust_crate_dir).expect("fail to guess class_name")
    });

    Opts {
        rust_input_path,
        dart_output_path: canon_path(&raw.dart_output),
        c_output_path: canon_path(&raw.c_output.unwrap_or_else(|| TODO)),
        rust_crate_dir,
        rust_output_path,
        class_name,
        dart_format_line_length: raw.dart_format_line_length.unwrap_or(80),
    }
}

fn fallback_rust_crate_dir(rust_input_path: &str) -> Result<String> {
    let mut dir_curr = Path::new(rust_input_path).parent()?;
    loop {
        let path_cargo_toml = dir_curr.join("Cargo.toml");

        if path_cargo_toml.exists() {
            return Ok(dir_curr.as_os_str().to_str()?.to_string());
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
        .parent()?
        .join("generated.rs")
        .to_str()
        .to_string())
}

fn fallback_class_name(rust_crate_dir: &str) -> Result<String> {
    let cargo_toml_path = Path::new(rust_crate_dir).join("Cargo.toml");
    let cargo_toml_content = fs::read_to_string(cargo_toml_path)?;

    let cargo_toml_value = cargo_toml_content.parse::<Value>()?;
    let package_name = cargo_toml_value.get("package")?.get("name")?.as_str()?;

    Ok(package_name.to_case(Case::Pascal))
}

fn canon_path(sub_path: &str) -> String {
    let mut path = env::current_dir().expect(&format!("fail to parse path: {}", sub_path));
    path.push(sub_path);
    path.into_os_string()
        .into_string()
        .expect(&format!("fail to parse path: {}", sub_path))
}

impl RawOpts {
    // pub fn read(config_path: &str) -> Self {
    //     let raw_config: Config =
    //         serde_yaml::from_str(&fs::read_to_string(config_path).unwrap()).unwrap();
    //     raw_config.canonicalize(PathBuf::from(config_path).parent().unwrap())
    // }

    // TODO
    // pub fn canonicalize(self, base_dir: &Path) -> Config {
    //     let canon_dir = |sub_path: &str| {
    //         let mut path = PathBuf::from(base_dir);
    //         path.push(sub_path);
    //         path.into_os_string().into_string().unwrap()
    //     };
    //
    //     Config {
    //         rust: ConfigRust {
    //             crate_dir: canon_dir(&self.rust.crate_dir),
    //             input_path: canon_dir(&self.rust.input_path),
    //             output_path: canon_dir(&self.rust.output_path),
    //         },
    //         dart: ConfigDart {
    //             output_path: canon_dir(&self.dart.output_path),
    //             output_class_name: self.dart.output_class_name,
    //             format_line_length: self.dart.format_line_length,
    //         },
    //         c: ConfigC {
    //             output_path: canon_dir(&self.c.output_path),
    //         },
    //     }
    // }
}

impl RawOpts {
    pub fn dart_api_class_name(&self) -> String {
        self.dart_output_class_name.clone()
    }

    pub fn dart_api_impl_class_name(&self) -> String {
        format!("_{}Impl", self.dart_output_class_name)
    }

    pub fn dart_wire_class_name(&self) -> String {
        format!("{}Wire", self.dart_output_class_name)
    }
}
