use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::ir::pack::IrPack;
use crate::library::commands::ffigen::{ffigen, FfigenArgs};

pub(crate) mod internal_config;
pub(super) mod spec_generator;

pub(crate) fn generate(ir_pack: &IrPack, context: WireDartGeneratorContext) -> anyhow::Result<()> {
    execute_ffigen()?;

    let spec = spec_generator::generate(ir_pack, context);
    todo!()
}

fn execute_ffigen() -> anyhow::Result<()> {
    ffigen(FfigenArgs {
        c_file_content: todo!(),
        dart_class_name: todo!(),
        llvm_path: todo!(),
        llvm_compiler_opts: todo!(),
        dart_root: todo!(),
    })?;
}
