use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::WireDartCodecDcoGeneratorContext;
use crate::codegen::ir::mir::ty::MirType;
use crate::library::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;

pub(crate) fn gen_decode_simple_type_cast(
    ty: MirType,
    context: WireDartCodecDcoGeneratorContext,
) -> String {
    let cast_type = ApiDartGenerator::new(ty, context.as_api_dart_context()).dart_api_type();
    format!("return raw as {cast_type};")
}
