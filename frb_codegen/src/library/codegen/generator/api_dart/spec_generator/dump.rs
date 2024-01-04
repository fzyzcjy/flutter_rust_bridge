use crate::codegen::generator::api_dart::spec_generator::base::{
    ApiDartGenerator, ApiDartGeneratorContext,
};
use crate::codegen::generator::misc::Direction;
use crate::codegen::ir::pack::IrPackComputedCache;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;
use fern::HashMap;
use serde::Serialize;
use std::collections::HashMap;
use strum::IntoEnumIterator;

#[derive(Serialize)]
pub(crate) struct ApiDartDumpInfo {
    types: Vec<ApiDartDumpInfoType>,
}

#[derive(Serialize)]
pub(crate) struct ApiDartDumpInfoType {
    safe_ident: String,
    dart_api_type: HashMap<Direction, String>,
}

pub(super) fn generate_dump_info(
    cache: &IrPackComputedCache,
    context: ApiDartGeneratorContext,
) -> ApiDartDumpInfo {
    ApiDartDumpInfo {
        types: cache
            .distinct_types
            .iter()
            .map(|ty| {
                let gen = ApiDartGenerator::new(ty.clone(), context);
                ApiDartDumpInfoType {
                    safe_ident: ty.safe_ident(),
                    dart_api_type: Direction::iter()
                        .map(|d| (d, gen.dart_api_type(d)))
                        .collect(),
                }
            })
            .collect(),
    }
}
