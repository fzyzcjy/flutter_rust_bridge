use crate::codegen::generator::api_dart::spec_generator::class::ty::ApiDartGeneratorClassTrait;
use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::codegen::ir::ty::IrType::RustAutoOpaque;
use crate::library::codegen::generator::api_dart::spec_generator::base::*;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;

impl<'a> ApiDartGeneratorClassTrait for RustAutoOpaqueApiDartGenerator<'a> {
    fn generate_class(&self) -> Option<ApiDartGeneratedClass> {
        let dart_api_type =
            ApiDartGenerator::new(RustAutoOpaque(self.ir.clone()), self.context).dart_api_type();

        Some(ApiDartGeneratedClass {
            namespace: self.ir.namespace.clone(),
            code: format!(
                "@sealed class {dart_api_type} extends RustAutoOpaque {{
                    {dart_api_type}.fromWire(dynamic wire): super.fromWire(wire, _kStaticData);
                    // TODO
                }}"
            ),
            needs_freezed: false,
            ..Default::default()
        })
    }
}
