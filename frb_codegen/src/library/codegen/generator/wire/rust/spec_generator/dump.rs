use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::{
    WireRustGenerator, WireRustGeneratorContext,
};
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::WireRustCodecCstGenerator;
use crate::codegen::ir::mir::pack::MirPackComputedCache;
use crate::library::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;
use crate::library::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
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
    wrapper_struct_name: Option<String>,
}

pub(super) fn generate_dump_info(
    cache: &MirPackComputedCache,
    context: WireRustGeneratorContext,
) -> WireRustDumpInfo {
    WireRustDumpInfo {
        types: cache
            .distinct_types
            .iter()
            .map(|ty| {
                let gen = WireRustGenerator::new(ty.clone(), context);
                let cst_gen = WireRustCodecCstGenerator::new(
                    ty.clone(),
                    context.as_wire_rust_codec_cst_context(),
                );
                WireRustDumpInfoType {
                    safe_ident: ty.safe_ident(),
                    rust_wire_type: Target::iter()
                        .map(|target| (target, cst_gen.rust_wire_type(target)))
                        .collect(),
                    rust_wire_modifier: Target::iter()
                        .map(|target| (target, cst_gen.rust_wire_modifier(target)))
                        .collect(),
                    wrapper_struct_name: gen.wrapper_struct_name(),
                }
            })
            .collect(),
    }
}
