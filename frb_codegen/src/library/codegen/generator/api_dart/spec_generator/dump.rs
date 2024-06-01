use crate::codegen::generator::api_dart::spec_generator::base::{
    ApiDartGenerator, ApiDartGeneratorContext,
};
use crate::codegen::ir::mir::pack::MirPackComputedCache;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct ApiDartDumpInfo {
    types: Vec<ApiDartDumpInfoType>,
}

#[derive(Serialize)]
pub(crate) struct ApiDartDumpInfoType {
    safe_ident: String,
    dart_api_type: String,
}

pub(super) fn generate_dump_info(
    cache: &MirPackComputedCache,
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
                    dart_api_type: gen.dart_api_type(),
                }
            })
            .collect(),
    }
}
