use crate::codegen::generator::api_dart::spec_generator::class::method::generate_api_methods;
use crate::codegen::generator::api_dart::spec_generator::class::misc::generate_class_extra_body;
use crate::codegen::generator::api_dart::spec_generator::class::ty::ApiDartGeneratorClassTrait;
use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::codegen::ir::mir::ty::rust_opaque::MirTypeRustOpaque;
use crate::library::codegen::generator::api_dart::spec_generator::base::*;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use crate::utils::namespace::NamespacedName;
use lazy_static::lazy_static;
use regex::Regex;

impl<'a> ApiDartGeneratorClassTrait for RustOpaqueApiDartGenerator<'a> {
    fn generate_class(&self) -> Option<ApiDartGeneratedClass> {
        let dart_entrypoint_class_name = &self.context.config.dart_entrypoint_class_name;
        let dart_api_instance = format!("{dart_entrypoint_class_name}.instance.api");

        let rust_api_type = self.mir.rust_api_type();
        let dart_api_type = ApiDartGenerator::new(self.mir.clone(), self.context).dart_api_type();

        let methods = generate_api_methods(
            &NamespacedName::new(
                self.mir.namespace.clone(),
                compute_api_method_query_name(&self.mir, self.context),
            ),
            self.context,
        )
        .join("\n");
        let extra_body =
            generate_class_extra_body(self.mir_type(), &self.context.mir_pack.dart_code_of_type);

        Some(ApiDartGeneratedClass {
            namespace: self.mir.namespace.clone(),
            class_name: dart_api_type.clone(),
            code: format!(
                "
                "
            ),
            needs_freezed: false,
            header: Default::default(),
        })
    }

    fn generate_extra_impl_code(&self) -> Option<String> {
        Some(format!(
            "
            // Rust type: {rust_api_type}
            @sealed class {dart_api_type} extends RustOpaque {{
                // Not to be used by end users
                {dart_api_type}.frbInternalDcoDecode(List<dynamic> wire):
                    super.frbInternalDcoDecode(wire, _kStaticData);

                // Not to be used by end users
                {dart_api_type}.frbInternalSseDecode(BigInt ptr, int externalSizeOnNative):
                    super.frbInternalSseDecode(ptr, externalSizeOnNative, _kStaticData);

                static final _kStaticData = RustArcStaticData(
                    rustArcIncrementStrongCount: {dart_api_instance}.rust_arc_increment_strong_count_{dart_api_type},
                    rustArcDecrementStrongCount: {dart_api_instance}.rust_arc_decrement_strong_count_{dart_api_type},
                    rustArcDecrementStrongCountPtr: {dart_api_instance}.rust_arc_decrement_strong_count_{dart_api_type}Ptr,
                );

                {methods}
                {extra_body}
            }}"
        ))
    }
}

fn compute_api_method_query_name(
    mir: &MirTypeRustOpaque,
    _context: ApiDartGeneratorContext,
) -> String {
    lazy_static! {
        static ref FILTER: Regex =
            Regex::new(r"^flutter_rust_bridge::for_generated::RustAutoOpaqueInner<(.*)>$").unwrap();
    }

    FILTER.replace_all(&mir.inner.0, "$1").to_string()
}
