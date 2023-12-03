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
        let dart_api_type_camel = dart_api_type.to_case(Case::Camel);

        Some(ApiDartGeneratedClass {
            namespace: self.ir.namespace.clone(),
            code: format!(
                "@sealed class {dart_api_type} extends RustOpaque {{
                    {dart_api_type}.fromWire(dynamic wire): this._fromRaw(wire[0], wire[1]);

                    {dart_api_type}._fromRaw(int ptr, int size): super.unsafe(ptr, size);

                    @override
                    OpaqueDropFnType get dropFn => {dart_api_instance}.dropOpaque{dart_api_type};
            
                    @override
                    OpaqueShareFnType get shareFn => {dart_api_instance}.shareOpaque{dart_api_type};
            
                    @override
                    OpaqueTypeFinalizer get staticFinalizer => {dart_api_instance}.{dart_api_type_camel}Finalizer;
                }}"
            ),
            needs_freezed: false,
            ..Default::default()
        })
    }
}
