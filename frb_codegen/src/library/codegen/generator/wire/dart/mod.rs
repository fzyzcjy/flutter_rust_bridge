use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::ir::pack::IrPack;
use crate::library::commands::ffigen::{ffigen, FfigenArgs};

pub(crate) mod internal_config;
pub(super) mod spec_generator;

pub(crate) fn generate(context: WireDartGeneratorContext) -> anyhow::Result<()> {
    let text_ffigen = execute_ffigen(context.config)?;

    let spec = spec_generator::generate(context);

    todo!()
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
