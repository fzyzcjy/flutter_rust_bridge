use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::WireDartCodecCstGenerator;
use crate::codegen::ir::mir::pack::MirPackComputedCache;
use crate::library::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
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
    cache: &MirPackComputedCache,
    context: WireDartGeneratorContext,
) -> WireDartDumpInfo {
    WireDartDumpInfo {
        types: cache
            .distinct_types
            .iter()
            .map(|ty| {
                let gen = WireDartCodecCstGenerator::new(
                    ty.clone(),
                    context.as_wire_dart_codec_cst_context(),
                );
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
