use crate::codegen::generator::api_dart::spec_generator::class::ty::ApiDartGeneratorClassTrait;
use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::library::codegen::generator::api_dart::spec_generator::base::*;

impl<'a> ApiDartGeneratorClassTrait for TraitDefApiDartGenerator<'a> {
    fn generate_class(&self) -> Option<ApiDartGeneratedClass> {
        Some(ApiDartGeneratedClass {
            namespace: self.mir.name.namespace.clone(),
            class_name: self.mir.name.name.clone(),
            code: format!(
                "
                abstract class {dart_api_type} {{
                    {methods}
                }}
                "
            ),
            needs_freezed: false,
            header: Default::default(),
        })
    }
}
