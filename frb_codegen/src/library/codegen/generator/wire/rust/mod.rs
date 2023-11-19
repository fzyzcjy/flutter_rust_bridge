use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::TargetOrCommon;
use crate::codegen::generator::wire::rust::internal_config::GeneratorWireRustInternalConfig;
use crate::codegen::generator::wire::rust::spec_generator::api2wire::WireRustOutputSpecApi2wire;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::misc::WireRustOutputSpecMisc;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::wire2api::WireRustOutputSpecWire2api;
use crate::codegen::generator::wire::rust::spec_generator::WireRustOutputSpec;
use crate::codegen::ir::pack::{IrPack, IrPackComputedCache};
use crate::codegen::ir::ty::IrType;
use crate::command_run;
use crate::library::commands::format_rust::format_rust;
use itertools::Itertools;

mod emitter;
pub(crate) mod internal_config;
pub(crate) mod spec_generator;
mod text_generator;

pub(crate) struct GeneratorWireRustOutput {
    pub extern_func_names: Vec<String>,
    pub extern_struct_names: Vec<String>,
}

pub(crate) fn generate(
    context: WireRustGeneratorContext,
) -> anyhow::Result<GeneratorWireRustOutput> {
    let spec = spec_generator::generate(context);
    let text = text_generator::generate(&spec, context.config)?;
    emitter::emit(text, context.config)?;
    Ok(compute_output(&spec))
}

fn compute_output(spec: &WireRustOutputSpec) -> GeneratorWireRustOutput {
    GeneratorWireRustOutput {
        // TODO originally from: `generated_rust.extern_func_names`
        extern_func_names: TODO,
        // TODO originally created via `get_c_struct_names`, should calc it from wire-rust layer, in analogy to `extern_func_names`
        extern_struct_names: TODO,
    }
}
