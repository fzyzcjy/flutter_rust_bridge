use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::{
    WireRustGenerator, WireRustGeneratorContext,
};
use crate::codegen::ir::pack::IrPackComputedCache;
use crate::library::codegen::generator::wire::rust::spec_generator::dart2rust::ty::WireRustGeneratorDart2RustTrait;
use crate::library::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;
use crate::library::codegen::generator::wire::rust::spec_generator::rust2dart::ty::WireRustGeneratorRust2DartTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;
use serde::Serialize;
use std::collections::HashMap;
use strum::IntoEnumIterator;

#[derive(Serialize)]
pub(crate) struct WireRustDumpInfo {
    types: Vec<WireRustDumpInfoType>,
}

#[derive(Serialize)]
pub(crate) struct WireRustDumpInfoType {
    safe_ident: String,
    rust_wire_type: HashMap<Target, String>,
    rust_wire_modifier: HashMap<Target, String>,
    intodart_type: String,
    wrapper_struct_name: Option<String>,
}

pub(super) fn generate_dump_info(
    cache: &IrPackComputedCache,
    context: WireRustGeneratorContext,
) -> WireRustDumpInfo {
    WireRustDumpInfo {
        types: cache
            .distinct_types
            .iter()
            .map(|ty| {
                let gen = WireRustGenerator::new(ty.clone(), context);
                WireRustDumpInfoType {
                    safe_ident: ty.safe_ident(),
                    rust_wire_type: Target::iter()
                        .map(|target| (target, gen.rust_wire_type(target)))
                        .collect(),
                    rust_wire_modifier: Target::iter()
                        .map(|target| (target, gen.rust_wire_modifier(target)))
                        .collect(),
                    intodart_type: gen.intodart_type(context.ir_pack),
                    wrapper_struct_name: gen.wrapper_struct_name(),
                }
            })
            .collect(),
    }
}
