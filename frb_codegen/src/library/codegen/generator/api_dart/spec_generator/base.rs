use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::ir::mir::ty::MirType::*;
use crate::codegen_generator_structs;
use enum_dispatch::enum_dispatch;

codegen_generator_structs!(
    #[enum_dispatch(ApiDartGeneratorInfoTrait)]
    #[enum_dispatch(ApiDartGeneratorClassTrait)]
    ApiDartGenerator
);

#[derive(Debug, Clone, Copy)]
pub(crate) struct ApiDartGeneratorContext<'a> {
    pub(crate) mir_pack: &'a MirPack,
    pub(crate) config: &'a GeneratorApiDartInternalConfig,
}
