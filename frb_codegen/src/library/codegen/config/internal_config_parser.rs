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
                    use_bridge_in_method: TODO,
                    wasm_enabled: TODO,
                    dart3: TODO,
                },
                rust: GeneratorRustInternalConfig {
                    rust_crate_dir: TODO,
                    rust_output_path: TODO,
                    inline_rust: TODO,
                },
                c: GeneratorCInternalConfig {
                    c_output_path: TODO,
                    llvm_path: TODO,
                    llvm_compiler_opts: TODO,
                    extra_headers: TODO,
                },
            },
            polisher: PolisherInternalConfig {
                extra_c_output_path: TODO,
                dart_format_line_length: TODO,
                add_mod_to_lib: TODO,
                build_runner: TODO,
                deps_check: TODO,
            },
        })
    }
}
