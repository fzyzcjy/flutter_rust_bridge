use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::rust2dart::ty::WireDartGeneratorRust2DartTrait;
use crate::codegen::ir::ty::rust_opaque::IrTypeRustOpaque;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;

impl<'a> WireDartGeneratorRust2DartTrait for RustOpaqueWireDartGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> String {
        generalized_rust_opaque_generate_impl_wire2api_body(self.ir.clone().into(), self.context)
    }
}

pub(super) fn generalized_rust_opaque_generate_impl_wire2api_body(
    ir: IrType,
    context: WireDartGeneratorContext,
) -> String {
    format!(
        "return {}.fromWire(raw);",
        ApiDartGenerator::new(ir, context.as_api_dart_context()).dart_api_type()
    )
}
