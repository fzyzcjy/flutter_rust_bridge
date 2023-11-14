use anyhow::Result;
use crate::codegen::Config;
use crate::codegen::config::internal_config::{GeneratorCInternalConfig, GeneratorDartInternalConfig, GeneratorInternalConfig, GeneratorRustInternalConfig, InternalConfig, ParserInternalConfig, PolisherInternalConfig};

impl InternalConfig {
    pub(crate) fn parse(config: Config) -> Result<Self> {
        Ok(InternalConfig {
            parser: ParserInternalConfig {
                rust_input_path: TODO,
                manifest_path: TODO,
            },
            generator: GeneratorInternalConfig {
                dart: GeneratorDartInternalConfig {
                    dart_output_path: TODO,
                    dart_decl_output_path: TODO,
                    dart_enums_style: TODO,
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
                    llvm_path: TODO,
                    llvm_compiler_opts: TODO,
                    extra_headers: config.extra_headers.unwrap_or_else(String::new),
                },
            },
            polisher: PolisherInternalConfig {
                extra_c_output_path: TODO,
                dart_format_line_length: TODO,
                add_mod_to_lib: config.add_mod_to_lib.unwrap_or(true),
                build_runner: config.build_runner.unwrap_or(true),
                deps_check: TODO,
            },
        })
    }
}
