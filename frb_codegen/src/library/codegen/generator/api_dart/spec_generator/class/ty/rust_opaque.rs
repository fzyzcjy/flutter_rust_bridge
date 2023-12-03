use crate::codegen::generator::api_dart::spec_generator::class::ty::ApiDartGeneratorClassTrait;
use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::api_dart::spec_generator::base::*;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use convert_case::{Case, Casing};
use IrType::RustOpaque;

impl<'a> ApiDartGeneratorClassTrait for RustOpaqueApiDartGenerator<'a> {
    fn generate_class(&self) -> Option<ApiDartGeneratedClass> {
        let dart_entrypoint_class_name = &self.context.config.dart_entrypoint_class_name;
        let dart_api_instance = format!("{dart_entrypoint_class_name}.instance.api");

        let dart_api_type =
            ApiDartGenerator::new(RustOpaque(self.ir.clone()), self.context).dart_api_type();

        Some(ApiDartGeneratedClass {
            namespace: self.ir.namespace.clone(),
            code: format!(
                "@sealed class {dart_api_type} extends RustOpaque {{
                    {dart_api_type}.fromWire(dynamic wire): super.fromWire(wire, _kStaticData);

                    static final _kStaticData = RustArcStaticData(
                        rustArcIncrementStrongCount: {dart_api_instance}.rust_arc_increment_strong_count_{dart_api_type},
                        rustArcDecrementStrongCount: {dart_api_instance}.rust_arc_decrement_strong_count_{dart_api_type},
                        rustArcDecrementStrongCountPtr: {dart_api_instance}.rust_arc_decrement_strong_count_{dart_api_type}Ptr,
                    );
                }}"
            ),
            needs_freezed: false,
            ..Default::default()
        })
    }
}
