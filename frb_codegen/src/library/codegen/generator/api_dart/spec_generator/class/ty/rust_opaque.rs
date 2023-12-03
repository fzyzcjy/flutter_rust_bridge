use crate::codegen::generator::api_dart::spec_generator::class::ty::ApiDartGeneratorClassTrait;
use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::codegen::ir::namespace::Namespace;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::api_dart::spec_generator::base::*;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;
use convert_case::{Case, Casing};
use IrType::RustOpaque;

impl<'a> ApiDartGeneratorClassTrait for RustOpaqueApiDartGenerator<'a> {
    fn generate_class(&self) -> Option<ApiDartGeneratedClass> {
        Some(generalized_rust_opaque_generate_class(
            self.ir.clone().into(),
            self.ir.namespace.clone(),
            self.context,
            "RustOpaque",
        ))
    }
}

pub(super) fn generalized_rust_opaque_generate_class(
    ir: IrType,
    namespace: Namespace,
    context: ApiDartGeneratorContext,
    base_class: &str,
) -> ApiDartGeneratedClass {
    let dart_entrypoint_class_name = &context.config.dart_entrypoint_class_name;
    let dart_api_instance = format!("{dart_entrypoint_class_name}.instance.api");

    let rust_api_type = ir.rust_api_type();
    let dart_api_type = ApiDartGenerator::new(ir, context).dart_api_type();

    ApiDartGeneratedClass {
        namespace,
        code: format!(
            "
            // Rust type: {rust_api_type}
            @sealed class {dart_api_type} extends {base_class} {{
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
    }
}
