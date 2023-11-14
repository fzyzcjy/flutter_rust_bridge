use std::path::{Path, PathBuf};
use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use log::debug;
use crate::codegen::Config;
use crate::codegen::config::internal_config::{DartOutputPaths, GeneratorCInternalConfig, GeneratorDartInternalConfig, GeneratorInternalConfig, GeneratorRustInternalConfig, InternalConfig, ParserInternalConfig, PolisherInternalConfig, RustOutputPaths};
use crate::utils::fp::Also;
use crate::utils::path_utils::{canonicalize_path, glob_path};

impl InternalConfig {
    pub(crate) fn parse(config: Config) -> Result<Self> {
        let base_dir = config.base_dir.map(PathBuf::from).unwrap_or_else(|| std::env::current_dir()?);
        debug!("InternalConfig.parse base_dir={base_dir}");

        let rust_crate_dir: PathBuf = config.rust_crate_dir.unwrap_or_else(|| TODO()).into();
        let manifest_path = rust_crate_dir.join("Cargo.toml");

        let dart_root = config.dart_root.map(PathBuf::from)
            .unwrap_or_else(|| default_dart_root(dart_output_path)?);

        let rust_input_path = glob_path(&config.rust_input);
        let rust_output_path = canonicalize_path(&config.rust_output, &base_dir);
        let c_output_path = canonicalize_path(&config.c_output, &base_dir);
        let duplicated_c_output_path = config.duplicated_c_output.unwrap_or_default()
            .into_iter().map(|p| canonicalize_path(&p, &base_dir)).collect();

        Ok(InternalConfig {
            parser: ParserInternalConfig {
                rust_input_path,
                manifest_path,
            },
            generator: GeneratorInternalConfig {
                dart: GeneratorDartInternalConfig {
                    dart_output_paths: DartOutputPaths {
                        dart_decl_output_path: TODO,
                        dart_impl_output_path: TODO,
                    },
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
                    llvm_path: config.llvm_path.unwrap_or_else(default_llvm_path)
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

fn default_llvm_path() -> Vec<String> {
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

fn default_dart_root(dart_output_path: &Path) -> Result<PathBuf> {
    let mut res = dart_output_path.to_owned();
    while res.pop() {
        if res.join("pubspec.yaml").is_file() {
            return Ok(res);
        }
    }
    Err(anyhow!("Root of Dart library could not be inferred from Dart output"))
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
