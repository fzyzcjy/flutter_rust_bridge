use crate::codegen::generator::dart_api::base::*;
use crate::codegen::generator::dart_api::class::field::{
    generate_field_default, generate_field_required_modifier,
};
use crate::codegen::generator::dart_api::class::ty::DartApiGeneratorClassTrait;
use crate::codegen::generator::dart_api::misc::{
    generate_dart_comments, generate_dart_maybe_implements_exception,
};
use crate::codegen::ir::field::IrField;
use crate::codegen::ir::ty::enumeration::{IrEnum, IrEnumMode, IrVariant, IrVariantKind};
use crate::codegen::ir::ty::structure::IrStruct;
use crate::library::codegen::generator::dart_api::decl::DartApiGeneratorDeclTrait;
use crate::utils::dart_keywords::make_string_keyword_safe;
use itertools::Itertools;

impl<'a> EnumRefDartApiGenerator<'a> {
    pub(super) fn generate_mode_simple(&self, src: &IrEnum) -> Option<String> {
        let comments = generate_dart_comments(&src.comments);

        let variants = src
            .variants()
            .iter()
            .map(|variant| self.generate_variant(variant))
            .collect_vec()
            .join("\n");

        Some(format!(
            "{}enum {} {{
                {}
            }}",
            comments, self.ir.ident.0, variants
        ))
    }

    fn generate_variant(&self, variant: &IrVariant) -> String {
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
