use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::base::{
    WireDartGenerator, WireDartGeneratorContext,
};
use crate::codegen::ir::pack::IrPackComputedCache;
use serde::Serialize;
use std::collections::HashMap;
use strum::IntoEnumIterator;

#[derive(Serialize)]
pub(crate) struct WireDartDumpInfo {
    types: Vec<WireDartDumpInfoType>,
}

#[derive(Serialize)]
pub(crate) struct WireDartDumpInfoType {
    safe_ident: String,
    dart_wire_type: HashMap<Target, String>,
}

pub(super) fn generate_dump_info(
    cache: IrPackComputedCache,
    context: WireDartGeneratorContext,
) -> WireDartDumpInfo {
    WireDartDumpInfo {
        types: cache
            .distinct_types
            .iter()
            .map(|ty| {
                let gen = WireDartGenerator::new(ty, context);
                WireDartDumpInfoType {
                    safe_ident: ty.safe_ident(),
                    dart_wire_type: Target::iter()
                        .map(|target| (target, gen.dart_wire_type(target)))
                        .collect(),
                }
            })
            .collect(),
    }
}
