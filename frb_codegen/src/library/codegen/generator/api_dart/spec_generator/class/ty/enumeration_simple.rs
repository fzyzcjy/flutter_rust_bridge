use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::codegen::generator::api_dart::spec_generator::misc::generate_dart_comments;
use crate::codegen::ir::mir::ty::enumeration::{MirEnum, MirEnumVariant};
use crate::library::codegen::generator::api_dart::spec_generator::base::*;
use crate::utils::basic_code::dart_header_code::DartHeaderCode;
use crate::utils::dart_keywords;
use itertools::Itertools;

impl<'a> EnumRefApiDartGenerator<'a> {
    pub(crate) fn generate_mode_simple(
        &self,
        src: &MirEnum,
        extra_body: &str,
        header: DartHeaderCode,
    ) -> Option<ApiDartGeneratedClass> {
        let comments = generate_dart_comments(&src.comments);

        let variants = src
            .variants()
            .iter()
            .map(|variant| self.generate_mode_simple_variant(variant))
            .collect_vec()
            .join("\n");

        let name = &self.mir.ident.0.name;

        Some(ApiDartGeneratedClass {
            namespace: src.name.namespace.clone(),
            class_name: name.clone(),
            code: format!(
                "{comments}enum {name} {{
                    {variants}
                    ;
                    {extra_body}
                }}",
            ),
            needs_freezed: false,
            header,
        })
    }

    fn generate_mode_simple_variant(&self, variant: &MirEnumVariant) -> String {
        let variant_name = if self.context.config.dart_enums_style {
            dart_keywords::escape(variant.name.dart_style())
        } else {
            variant.name.rust_style().to_string()
        };

        format!(
            "{}{},",
            generate_dart_comments(&variant.comments),
            variant_name
        )
    }
}
