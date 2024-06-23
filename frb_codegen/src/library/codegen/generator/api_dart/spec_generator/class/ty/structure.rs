use crate::codegen::generator::api_dart::spec_generator::class::method::{
    dart_constructor_postfix, generate_api_methods, GenerateApiMethodConfig,
};
use crate::codegen::generator::api_dart::spec_generator::class::misc::generate_class_extra_body;
use crate::codegen::generator::api_dart::spec_generator::class::ty::ApiDartGeneratorClassTrait;
use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::codegen::generator::api_dart::spec_generator::misc::{
    generate_dart_comments, generate_dart_metadata,
};
use crate::codegen::ir::mir::ty::MirType;
use crate::library::codegen::generator::api_dart::spec_generator::base::*;

impl<'a> ApiDartGeneratorClassTrait for StructRefApiDartGenerator<'a> {
    fn generate_class(&self) -> Option<ApiDartGeneratedClass> {
        let src = self.mir.get(self.context.mir_pack);
        let comments = generate_dart_comments(&src.comments);
        let metadata = generate_dart_metadata(&src.dart_metadata);

        let constructor_postfix = dart_constructor_postfix(
            &src.name.name,
            &self.context.mir_pack.funcs_with_impl(),
            self.context,
        );
        let class_name = &self.mir.ident.0.name;

        let methods = generate_api_methods(
            &MirType::StructRef(self.mir.clone()),
            self.context,
            &GenerateApiMethodConfig::COMBINED,
            class_name,
        );
        let extra_code =
            generate_class_extra_body(self.mir_type(), &self.context.mir_pack.dart_code_of_type);
        let extra_body = &extra_code.body;

        Some(ApiDartGeneratedClass {
            namespace: src.name.namespace.clone(),
            class_name: class_name.to_owned(),
            code: if src.using_freezed() {
                self.generate_mode_freezed(
                    src,
                    &comments,
                    &metadata,
                    &methods,
                    constructor_postfix,
                    extra_body,
                    class_name,
                )
            } else {
                self.generate_mode_non_freezed(
                    src,
                    &comments,
                    &metadata,
                    &methods,
                    constructor_postfix,
                    extra_body,
                    class_name,
                )
            },
            needs_freezed: src.using_freezed(),
            header: methods.header + extra_code.header,
        })
    }
}
