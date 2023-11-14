use std::path::PathBuf;
use anyhow::Result;
use itertools::Itertools;
use log::debug;
use crate::codegen::Config;
use crate::codegen::config::internal_config::{GeneratorCInternalConfig, GeneratorDartInternalConfig, GeneratorInternalConfig, GeneratorRustInternalConfig, InternalConfig, ParserInternalConfig, PolisherInternalConfig};

impl InternalConfig {
    pub(crate) fn parse(config: Config) -> Result<Self> {
        let base_dir = config.base_dir.map(PathBuf::from).unwrap_or_else(|| std::env::current_dir()?);
        debug!("InternalConfig.parse base_dir={base_dir}");

        Ok(InternalConfig {
            parser: ParserInternalConfig {
                rust_input_path: TODO,
                manifest_path: TODO,
            },
            generator: GeneratorInternalConfig {
                dart: GeneratorDartInternalConfig {
                    dart_output_path: TODO,
                    dart_decl_output_path: TODO,
                    dart_enums_style: config.dart_enums_style.unwrap_or(false),
                    class_name: TODO,
                    dart_root: TODO,
                    use_bridge_in_method: config.use_bridge_in_method.unwrap_or(true),
                    wasm_enabled: config.wasm.unwrap_or(false),
                    dart3: config.dart3.unwrap_or(true),
                },
                rust: GeneratorRustInternalConfig {
                    rust_crate_dir: TODO,
                    rust_output_path: TODO,
                    inline_rust: config.inline_rust.unwrap_or(false),
                },
                c: GeneratorCInternalConfig {
                    c_output_path: TODO,
                    llvm_path: config.llvm_path.unwrap_or_else(get_default_llvm_path)
                        .into_iter().map(PathBuf::from).collect_vec(),
                    llvm_compiler_opts: config.llvm_compiler_opts.unwrap_or_else(String::new),
                    extra_headers: config.extra_headers.unwrap_or_else(String::new),
                },
            },
            polisher: PolisherInternalConfig {
                duplicate_c_output_path: TODO,
                dart_format_line_length: config.dart_format_line_length.unwrap_or(80),
                add_mod_to_lib: config.add_mod_to_lib.unwrap_or(true),
                build_runner: config.build_runner.unwrap_or(true),
                deps_check: config.deps_check.unwrap_or(true),
            },
        })
    }
}

fn get_default_llvm_path() -> Vec<String> {
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
