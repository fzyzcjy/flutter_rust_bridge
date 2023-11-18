use crate::codegen::generator::api_dart::base::*;
use crate::codegen::generator::api_dart::class::ty::ApiDartGeneratorClassTrait;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::api_dart::info::ApiDartGeneratorInfoTrait;
use IrType::RustOpaque;

impl<'a> ApiDartGeneratorClassTrait for RustOpaqueApiDartGenerator<'a> {
    fn generate_class(&self) -> Option<String> {
        let dart_api_instance_name = &self.context.config.dart_api_instance_name;
        let dart_api_class_name = &self.context.config.dart_api_class_name;
        let dart_api_type =
            ApiDartGenerator::new(RustOpaque(self.ir.clone()), self.context).dart_api_type();

        let (field, param) = if self.context.config.use_bridge_in_method {
            (
                format!("final {dart_api_class_name} bridge;"),
                ", this.bridge",
            )
        } else {
            (String::new(), "")
        };

        Some(format!(
            "@sealed class {dart_api_type} extends FrbOpaque {{
                {field}

                {dart_api_type}.fromRaw(int ptr, int size {param}): super.unsafe(ptr, size);

                @override
                DropFnType get dropFn => {dart_api_instance_name}.dropOpaque{dart_api_type};
        
                @override
                ShareFnType get shareFn => {dart_api_instance_name}.shareOpaque{dart_api_type};
        
                @override
                OpaqueTypeFinalizer get staticFinalizer => {dart_api_instance_name}.{dart_api_type}Finalizer;
            }}"
        ))
    }
}
