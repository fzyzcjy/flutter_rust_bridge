use crate::codegen::generator::misc::OutputTexts;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::ir::pack::IrPackComputedCache;

pub(crate) mod internal_config;
pub(crate) mod spec_generator;
mod text_generator;

pub(crate) struct GeneratorWireRustOutput {
    pub output_texts: OutputTexts,
    pub extern_func_names: Vec<String>,
    pub extern_struct_names: Vec<String>,
}

pub(crate) fn generate(
    context: WireRustGeneratorContext,
) -> anyhow::Result<GeneratorWireRustOutput> {
    let spec = spec_generator::generate(context);
    let text = text_generator::generate(&spec, context.config)?;

    Ok(GeneratorWireRustOutput {
        extern_func_names: text.extern_func_names,
        extern_struct_names: spec.misc.extern_struct_names,
    })
}
