use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::spec_generator::WireRustOutputSpec;
use crate::codegen::generator::wire::rust::GeneratorWireRustOutput;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::IrType;
use crate::if_then_some;

pub(super) fn compute_output(
    spec: &WireRustOutputSpec,
    ir_pack: &IrPack,
) -> GeneratorWireRustOutput {
    GeneratorWireRustOutput {
        // TODO originally from: `generated_rust.extern_func_names`
        extern_func_names: TODO,
        extern_struct_names: get_c_struct_names(ir_pack),
    }
}

fn get_c_struct_names(ir_pack: &IrPack) -> Vec<String> {
    let distinct_types = ir_pack.distinct_types(true, true);

    distinct_types
        .iter()
        .filter(|ty| matches!(&ty, IrType::StructRef(_)))
        .map(|ty| ty.rust_wire_type(Target::Io))
        .collect()
}
