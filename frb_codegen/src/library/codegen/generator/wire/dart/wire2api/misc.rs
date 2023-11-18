use crate::codegen::generator::api_dart::base::ApiDartGenerator;
use crate::codegen::generator::wire::dart::base::WireDartGeneratorContext;
use crate::codegen::ir::ty::IrType;

pub(super) fn gen_wire2api_simple_type_cast(
    ty: IrType,
    context: WireDartGeneratorContext,
) -> String {
    let cast_type = ApiDartGenerator::new(ty, context.as_api_dart_context()).dart_api_type();
    format!("return raw as {cast_type};")
}
