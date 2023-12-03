use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::codegen::generator::api_dart::spec_generator::misc::generate_dart_comments;
use crate::codegen::ir::ty::enumeration::{IrEnum, IrVariant};
use crate::library::codegen::generator::api_dart::spec_generator::base::*;
use crate::utils::dart_keywords::make_string_keyword_safe;
use itertools::Itertools;

impl<'a> EnumRefApiDartGenerator<'a> {
    pub(crate) fn generate_mode_simple(&self, src: &IrEnum) -> Option<ApiDartGeneratedClass> {
        let comments = generate_dart_comments(&src.comments);

        let variants = src
            .variants()
            .iter()
            .map(|variant| self.generate_mode_simple_variant(variant))
            .collect_vec()
            .join("\n");

        Some(ApiDartGeneratedClass {
            namespace: src.name.namespace.clone(),
            deduplicate_key: TODO,
            code: format!(
                "{}enum {} {{
                    {}
                }}",
                comments, self.ir.ident.0.name, variants
            ),
            needs_freezed: false,
            ..Default::default()
        })
    }

    fn generate_mode_simple_variant(&self, variant: &IrVariant) -> String {
        let variant_name = if self.context.config.dart_enums_style {
            make_string_keyword_safe(variant.name.dart_style())
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
