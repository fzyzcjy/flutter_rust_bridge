use crate::codegen::generator::dart_api::base::*;
use crate::codegen::generator::dart_api::class::DartApiGeneratorClassTrait;

impl<'a> DartApiGeneratorClassTrait for RustOpaqueDartApiGenerator<'a> {
    fn generate_class(&self) -> Option<String> {
        let bridge = self.context.config.get_dart_api_bridge_name();
        let bridge_class = &self.context.config.dart_api_class_name;
        let (field, param) = if self.context.config.bridge_in_method {
            (format!("final {bridge_class} bridge;"), "this.bridge")
        } else {
            (String::new(), "")
        };
        let dart_api_type = self.ir.dart_api_type();
        Some(format!(
            "@sealed class {dart_api_type} extends FrbOpaque {{
                {field}

                {dart_api_type}.fromRaw(int ptr, int size, {param}): super.unsafe(ptr, size);

                @override
                DropFnType get dropFn => {bridge}.dropOpaque{dart_api_type};
        
                @override
                ShareFnType get shareFn => {bridge}.shareOpaque{dart_api_type};
        
                @override
                OpaqueTypeFinalizer get staticFinalizer => {bridge}.{dart_api_type}Finalizer;
            }}"
        ))
    }
}
