use crate::codegen::generator::api_dart::spec_generator::class::method::{generate_api_methods, GenerateApiMethodMode, GeneratedApiMethods};
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
        let Info {
            dart_api_type,
            methods,
        } = self.compute_info(GenerateApiMethodMode::SeparatedDecl);
        let methods_str = &methods.code;

        let rust_api_type = self.mir.rust_api_type();

        let extra_body =
            generate_class_extra_body(self.mir_type(), &self.context.mir_pack.dart_code_of_type);

        Some(ApiDartGeneratedClass {
            namespace: self.mir.namespace.clone(),
            class_name: dart_api_type.clone(),
            code: format!(
                "
                // Rust type: {rust_api_type}
                abstract class {dart_api_type} {{
                    {methods_str}

                    void dispose();

                    bool get isDisposed;

                    {extra_body}
                }}
                "
            ),
            needs_freezed: false,
            header: methods.header,
        })
    }

    fn generate_extra_impl_code(&self) -> Option<String> {
        let Info {
            dart_api_type,
            methods,
        } = self.compute_info(GenerateApiMethodMode::SeparatedImpl);
        let methods_str = &methods.code;

        let dart_api_type_impl = format!("{dart_api_type}Impl");

        let dart_entrypoint_class_name = &self.context.config.dart_entrypoint_class_name;
        let dart_api_instance = format!("{dart_entrypoint_class_name}.instance.api");

        Some(format!(
            "
            @sealed class {dart_api_type_impl} extends RustOpaque implements {dart_api_type} {{
                // Not to be used by end users
                {dart_api_type_impl}.frbInternalDcoDecode(List<dynamic> wire):
                    super.frbInternalDcoDecode(wire, _kStaticData);

                // Not to be used by end users
                {dart_api_type_impl}.frbInternalSseDecode(BigInt ptr, int externalSizeOnNative):
                    super.frbInternalSseDecode(ptr, externalSizeOnNative, _kStaticData);

                static final _kStaticData = RustArcStaticData(
                    rustArcIncrementStrongCount: {dart_api_instance}.rust_arc_increment_strong_count_{dart_api_type},
                    rustArcDecrementStrongCount: {dart_api_instance}.rust_arc_decrement_strong_count_{dart_api_type},
                    rustArcDecrementStrongCountPtr: {dart_api_instance}.rust_arc_decrement_strong_count_{dart_api_type}Ptr,
                );

                {methods_str}
            }}"
        ))
    }
}

impl RustOpaqueApiDartGenerator<'_> {
    fn compute_info(&self, mode: GenerateApiMethodMode) -> Info {
        let dart_api_type = ApiDartGenerator::new(self.mir.clone(), self.context).dart_api_type();

        let methods = generate_api_methods(
            &NamespacedName::new(
                self.mir.namespace.clone(),
                compute_api_method_query_name(&self.mir, self.context),
            ),
            self.context,
            mode,
        );

        Info {
            dart_api_type,
            methods,
        }
    }
}

struct Info {
    dart_api_type: String,
    methods: GeneratedApiMethods,
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
