use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::library::commands::ffigen::{ffigen, FfigenArgs};

pub(super) fn generate(config: &GeneratorWireDartInternalConfig) -> anyhow::Result<String> {
    let raw = execute_ffigen(config)?;
}

fn execute_ffigen(config: &GeneratorWireDartInternalConfig) -> anyhow::Result<String> {
    ffigen(FfigenArgs {
        c_file_content: &config.c_file_content,
        dart_class_name: &config.dart_wire_class_name,
        llvm_path: &config.llvm_path,
        llvm_compiler_opts: &config.llvm_compiler_opts,
        dart_root: &config.dart_root,
    })
}
