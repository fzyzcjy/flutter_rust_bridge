use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;

pub(crate) fn gen_wire2api_simple_type_cast(
    ty: IrType,
    context: WireDartGeneratorContext,
) -> String {
    let cast_type = ApiDartGenerator::new(ty, context.as_api_dart_context()).dart_api_type();
    format!("return raw as {cast_type};")
}
