use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::spec_generator::WireRustOutputSpec;
use crate::codegen::generator::wire::rust::GeneratorWireRustOutput;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::IrType;

pub(super) fn compute_output(spec: &WireRustOutputSpec) -> GeneratorWireRustOutput {
    GeneratorWireRustOutput {
        // TODO originally from: `generated_rust.extern_func_names`
        extern_func_names: TODO,
        // TODO originally created via `get_c_struct_names`, should calc it from wire-rust layer, in analogy to `extern_func_names`
        extern_struct_names: TODO,
    }
}

fn get_c_struct_names(ir_pack: &IrPack) -> Vec<String> {
    ir_pack
        .distinct_types(true, true)
        .iter()
        .filter_map(|ty| {
            if let IrType::StructRef(_) = ty {
                Some(ty.rust_wire_type(Target::Io))
            } else {
                None
            }
        })
        .collect()
}
