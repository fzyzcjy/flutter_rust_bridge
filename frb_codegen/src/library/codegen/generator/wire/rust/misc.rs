use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::{
    WireRustGenerator, WireRustGeneratorContext,
};
use crate::codegen::generator::wire::rust::spec_generator::WireRustOutputSpec;
use crate::codegen::generator::wire::rust::GeneratorWireRustOutput;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::IrType;
use crate::if_then_some;
use crate::library::codegen::generator::wire::rust::spec_generator::wire2api::ty::WireRustGeneratorWire2apiTrait;

pub(super) fn compute_output(
    spec: &WireRustOutputSpec,
    context: WireRustGeneratorContext,
) -> GeneratorWireRustOutput {
    GeneratorWireRustOutput {
        // TODO originally from: `generated_rust.extern_func_names`
        extern_func_names: TODO,
        extern_struct_names: get_c_struct_names(context),
    }
}

fn get_c_struct_names(context: WireRustGeneratorContext) -> Vec<String> {
    let distinct_types = context.ir_pack.distinct_types(true, true);

    distinct_types
        .iter()
        .filter(|ty| matches!(&ty, IrType::StructRef(_)))
        .map(|ty| WireRustGenerator::new(ty.clone(), context).rust_wire_type(Target::Io))
        .collect()
}
