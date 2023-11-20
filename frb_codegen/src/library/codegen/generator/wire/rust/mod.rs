use crate::codegen::generator::misc::PathTexts;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::ir::pack::IrPackComputedCache;

pub(crate) mod internal_config;
pub(crate) mod spec_generator;
mod text_generator;

pub(crate) struct GeneratorWireRustOutput {
    pub output_texts: PathTexts,
    pub extern_func_names: Vec<String>,
    pub extern_struct_names: Vec<String>,
}

pub(crate) fn generate(
    context: WireRustGeneratorContext,
) -> anyhow::Result<GeneratorWireRustOutput> {
    let spec = spec_generator::generate(context);
    let text = text_generator::generate(&spec, context.config)?;

    Ok(GeneratorWireRustOutput {
        output_texts: PathTexts::new_from_targets(&context.config.rust_output_path, &text.text),
        extern_func_names: text.extern_func_names,
        extern_struct_names: spec.misc.extern_struct_names,
    })
}
