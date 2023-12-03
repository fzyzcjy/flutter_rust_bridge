use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::rust2dart::ty::rust_opaque::generalized_rust_opaque_generate_impl_wire2api_body;
use crate::codegen::generator::wire::dart::spec_generator::rust2dart::ty::WireDartGeneratorRust2DartTrait;
use crate::library::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;

impl<'a> WireDartGeneratorRust2DartTrait for RustAutoOpaqueWireDartGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> String {
        generalized_rust_opaque_generate_impl_wire2api_body(self.ir.clone().into(), self.context)
    }
}
