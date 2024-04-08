use crate::codegen::generator::api_dart::spec_generator::class::method::{
    dart_constructor_postfix, generate_api_methods,
};
use crate::codegen::generator::api_dart::spec_generator::class::misc::generate_class_extra_body;
use crate::codegen::generator::api_dart::spec_generator::class::ty::ApiDartGeneratorClassTrait;
use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::codegen::generator::api_dart::spec_generator::misc::{
    generate_dart_comments, generate_dart_metadata,
};
use crate::library::codegen::generator::api_dart::spec_generator::base::*;

impl<'a> ApiDartGeneratorClassTrait for StructRefApiDartGenerator<'a> {
    fn generate_class(&self) -> Option<ApiDartGeneratedClass> {
        let src = self.ir.get(self.context.ir_pack);
        let comments = generate_dart_comments(&src.comments);
        let metadata = generate_dart_metadata(&src.dart_metadata);

        let methods = generate_api_methods(&src.name, self.context);
        let extra_body =
            generate_class_extra_body(self.ir_type(), &self.context.ir_pack.dart_code_of_type);

        let constructor_postfix = dart_constructor_postfix(&src.name, &self.context.ir_pack.funcs);

        Some(ApiDartGeneratedClass {
            namespace: src.name.namespace.clone(),
            code: if src.using_freezed() {
                self.generate_mode_freezed(
                    src,
                    &comments,
                    &metadata,
                    &methods,
                    constructor_postfix,
                    &extra_body,
                )
            } else {
                self.generate_mode_non_freezed(
                    src,
                    &comments,
                    &metadata,
                    &methods,
                    constructor_postfix,
                    &extra_body,
                )
            },
            needs_freezed: src.using_freezed(),
            ..Default::default()
        })
    }
}
