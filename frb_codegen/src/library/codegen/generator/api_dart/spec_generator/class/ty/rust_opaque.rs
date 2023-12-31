use crate::codegen::generator::api_dart::spec_generator::class::method::generate_api_methods;
use crate::codegen::generator::api_dart::spec_generator::class::ty::ApiDartGeneratorClassTrait;
use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::codegen::ir::namespace::NamespacedName;
use crate::codegen::ir::ty::rust_opaque::IrTypeRustOpaque;
use crate::library::codegen::generator::api_dart::spec_generator::base::*;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;
use lazy_static::lazy_static;
use regex::Regex;

impl<'a> ApiDartGeneratorClassTrait for RustOpaqueApiDartGenerator<'a> {
    fn generate_class(&self) -> Option<ApiDartGeneratedClass> {
        let dart_entrypoint_class_name = &self.context.config.dart_entrypoint_class_name;
        let dart_api_instance = format!("{dart_entrypoint_class_name}.instance.api");

        let rust_api_type = self.ir.rust_api_type();
        let dart_api_type = ApiDartGenerator::new(self.ir.clone(), self.context).dart_api_type();

        let methods = generate_api_methods(
            &NamespacedName::new(
                self.ir.namespace.clone(),
                compute_api_method_query_name(&self.ir, self.context),
            ),
            self.context,
        )
        .join("\n");

        Some(ApiDartGeneratedClass {
            namespace: self.ir.namespace.clone(),
            code: format!(
                "
            // Rust type: {rust_api_type}
            @sealed class {dart_api_type} extends RustOpaque {{
                {dart_api_type}.dcoDecode(List<dynamic> wire): super.dcoDecode(wire, _kStaticData);

                {dart_api_type}.sseDecode(int ptr, int externalSizeOnNative):
                    super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

                static final _kStaticData = RustArcStaticData(
                    rustArcIncrementStrongCount: {dart_api_instance}.rust_arc_increment_strong_count_{dart_api_type},
                    rustArcDecrementStrongCount: {dart_api_instance}.rust_arc_decrement_strong_count_{dart_api_type},
                    rustArcDecrementStrongCountPtr: {dart_api_instance}.rust_arc_decrement_strong_count_{dart_api_type}Ptr,
                );

                {methods}
            }}"
            ),
            needs_freezed: false,
            ..Default::default()
        })
    }
}

fn compute_api_method_query_name(
    ir: &IrTypeRustOpaque,
    _context: ApiDartGeneratorContext,
) -> String {
    lazy_static! {
        static ref FILTER: Regex = Regex::new(r"^std::sync::RwLock<(.*)>$").unwrap();
    }

    let inner_dart_api_type: String = ir.inner.rust_api_type();
    FILTER.replace_all(&inner_dart_api_type, "$1").to_string()
}
