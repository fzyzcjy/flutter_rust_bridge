use crate::codegen::generator::dart_api::base::*;
use crate::codegen::generator::dart_api::class::DartApiGeneratorClassTrait;

impl<'a> DartApiGeneratorClassTrait for RustOpaqueDartApiGenerator<'a> {
    fn generate_class(&self) -> Option<String> {
        todo!()
        // let bridge = self.context.config.get_dart_api_bridge_name();
        // let bridge_class = self.context.config.dart_api_class_name();
        // let (field, param) = if self.context.config.bridge_in_method {
        //     (format!("final {bridge_class} bridge;"), "this.bridge")
        // } else {
        //     (String::new(), "")
        // };
        // Some(format!(
        //     "@sealed class {0} extends FrbOpaque {{
        //         {field}
        //         {0}.fromRaw(int ptr, int size, {param}) : super.unsafe(ptr, size);
        //         @override
        //         DropFnType get dropFn => {bridge}.dropOpaque{0};
        //
        //         @override
        //         ShareFnType get shareFn => {bridge}.shareOpaque{0};
        //
        //         @override
        //         OpaqueTypeFinalizer get staticFinalizer => {bridge}.{0}Finalizer;
        //     }}",
        //     self.ir.dart_api_type()
        // ))
    }
}
