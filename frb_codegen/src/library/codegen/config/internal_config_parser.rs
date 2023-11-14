use std::fmt::format;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{anyhow, bail, Context, ensure, Result};
use itertools::Itertools;
use log::debug;
use crate::codegen::Config;
use crate::codegen::config::internal_config::{DartOutputPathPack, GeneratorCInternalConfig, GeneratorDartInternalConfig, GeneratorInternalConfig, GeneratorRustInternalConfig, InternalConfig, Namespace, ParserInternalConfig, PolisherInternalConfig, RustInputPathPack, RustOutputPaths};
use crate::utils::fp::Also;
use crate::utils::path_utils::{canonicalize_path, find_parent_dir_with_file, glob_path, path_to_string};

impl InternalConfig {
    pub(crate) fn parse(config: Config) -> Result<Self> {
        let base_dir = config.base_dir.map(PathBuf::from).unwrap_or_else(|| std::env::current_dir()?);
        debug!("InternalConfig.parse base_dir={base_dir}");

        let rust_input_path_pack = compute_rust_input_path_pack(&config.rust_input, &base_dir)?;

        let rust_output_path = canonicalize_path(&config.rust_output.map(PathBuf::from)
            .unwrap_or_else(|| fallback_rust_output_path(rust_input_path_pack.one_rust_input_path())), &base_dir);

        let dart_output_dir: PathBuf = config.dart_output.into();
        let dart_output_path_pack = compute_dart_output_path_pack(dart_output_dir, &rust_input_path_pack);

        let c_output_path = canonicalize_path(&config.c_output, &base_dir);
        let duplicated_c_output_path = config.duplicated_c_output.unwrap_or_default()
            .into_iter().map(|p| canonicalize_path(&p, &base_dir)).collect();

        let rust_crate_dir: PathBuf = config.rust_crate_dir.map(PathBuf::from)
            .unwrap_or_else(|| fallback_rust_crate_dir(rust_input_path_pack.one_rust_input_path())?);
        let manifest_path = rust_crate_dir.join("Cargo.toml");
        let dart_root = config.dart_root.map(PathBuf::from)
            .unwrap_or_else(|| fallback_dart_root(&dart_output_dir)?);

        Ok(InternalConfig {
            parser: ParserInternalConfig {
                rust_input_path_pack,
                manifest_path,
            },
            generator: GeneratorInternalConfig {
                dart: GeneratorDartInternalConfig {
                    dart_output_path_pack,
                    dart_enums_style: config.dart_enums_style.unwrap_or(false),
                    class_name: TODO,
                    dart_root,
                    use_bridge_in_method: config.use_bridge_in_method.unwrap_or(true),
                    wasm_enabled: config.wasm.unwrap_or(false),
                    dart3: config.dart3.unwrap_or(true),
                },
                rust: GeneratorRustInternalConfig {
                    rust_crate_dir,
                    rust_output_path,
                    inline_rust: config.inline_rust.unwrap_or(false),
                },
                c: GeneratorCInternalConfig {
                    c_output_path,
                    llvm_path: config.llvm_path.unwrap_or_else(fallback_llvm_path)
                        .into_iter().map(PathBuf::from).collect_vec(),
                    llvm_compiler_opts: config.llvm_compiler_opts.unwrap_or_else(String::new),
                    extra_headers: config.extra_headers.unwrap_or_else(String::new),
                },
            },
            polisher: PolisherInternalConfig {
                duplicated_c_output_path,
                dart_format_line_length: config.dart_format_line_length.unwrap_or(80),
                add_mod_to_lib: config.add_mod_to_lib.unwrap_or(true),
                build_runner: config.build_runner.unwrap_or(true),
                deps_check: config.deps_check.unwrap_or(true),
            },
        })
    }
}

impl RustInputPathPack {
    fn one_rust_input_path(&self) -> &Path { self.rust_input_path.iter().next().unwrap().1 }
}

fn compute_rust_input_path_pack(raw_rust_input: &str, base_dir: &Path) -> Result<RustInputPathPack> {
    let paths = glob_path(raw_rust_input, base_dir);

    let pack = RustInputPathPack {
        rust_input_path: paths.into_iter()
            .map(|path| (compute_namespace_from_rust_input_path(&path), path))
            .collect(),
    };

    ensure!(!pack.rust_input_path.is_empty());
    ensure!(
        !pack.rust_input_path.values().any(|p| path_to_string(p)?.contains("lib.rs")),
        "Do not use `lib.rs` as a Rust input. Please put code to be generated in something like `api.rs`.",
    );

    OK(pack)
}

fn compute_namespace_from_rust_input_path(rust_input_path: &Path) -> Namespace {
    TODO
}

fn compute_dart_output_path_pack(dart_output_dir: PathBuf, rust_input_path_pack: &RustInputPathPack) -> DartOutputPathPack {
    DartOutputPathPack {
        dart_decl_output_path: rust_input_path_pack.rust_input_path.keys()
            .map(|namespace| (namespace.to_owned(), dart_output_dir.join(compute_dart_decl_output_filename(namespace))))
            .collect(),
        dart_impl_output_path: dart_output_dir.join("bridge_generated.dart"),
    }
}

fn compute_dart_decl_output_filename(namespace: &Namespace) -> String {
    format!("{}.dart", TODO)
}

fn fallback_llvm_path() -> Vec<String> {
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
}

fn fallback_dart_root(dart_output_dir: &Path) -> Result<PathBuf> {
    find_parent_dir_with_file(dart_output_dir, "pubspec.yaml")
}

fn fallback_rust_crate_dir(rust_input_path: &Path) -> Result<PathBuf> {
    find_parent_dir_with_file(rust_input_path, "Cargo.toml")
}

fn fallback_rust_output_path(rust_input_path: &Path) -> PathBuf {
    rust_input_path.with_file_name("bridge_generated.rs")
}

#[cfg(test)]
mod tests {
    use crate::codegen::Config;
    use crate::codegen::config::internal_config::InternalConfig;

    #[test]
    fn test_parse_paths_simple() -> anyhow::Result<()> {
        let output = InternalConfig::parse(Config {
            rust_input: Some(vec!["api.rs".to_string()]),
            dart_output: Some("api.dart".to_string()),
            ..Default::default()
        })?;

        assert_eq!(output.parser.rust_input_path, todo);
        assert_eq!(output.generator.rust.rust_output_path, todo);
        assert_eq!(output.generator.dart.dart_output_path, todo);
        assert_eq!(output.generator.c.c_output_path, todo);

        Ok(())
    }

    #[test]
    fn test_parse_paths_multi_literal_input() -> anyhow::Result<()> {
        let output = InternalConfig::parse(Config {
            rust_input: Some(vec!["api_1.rs".to_string(), "api_2.rs".to_string()]),
            dart_output: Some("api.dart".to_string()),
            ..Default::default()
        })?;

        assert_eq!(output.parser.rust_input_path, todo);
        assert_eq!(output.generator.rust.rust_output_path, todo);
        assert_eq!(output.generator.dart.dart_output_path, todo);
        assert_eq!(output.generator.c.c_output_path, todo);

        Ok(())
    }

    #[test]
    fn test_parse_paths_regex_input() -> anyhow::Result<()> {
        let output = InternalConfig::parse(Config {
            rust_input: Some(vec!["api.*.rs".to_string()]),
            dart_output: Some("api.dart".to_string()),
            ..Default::default()
        })?;

        assert_eq!(output.parser.rust_input_path, todo);
        assert_eq!(output.generator.rust.rust_output_path, todo);
        assert_eq!(output.generator.dart.dart_output_path, todo);
        assert_eq!(output.generator.c.c_output_path, todo);

        Ok(())
    }
}
