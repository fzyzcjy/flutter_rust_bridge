use crate::codegen::generator::api_dart::spec_generator::class::field::{
    generate_field_default, generate_field_required_modifier,
};
use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::codegen::generator::api_dart::spec_generator::misc::{
    generate_dart_comments, generate_dart_maybe_implements_exception,
};
use crate::codegen::ir::mir::field::MirField;
use crate::codegen::ir::mir::ty::enumeration::{MirEnum, MirEnumVariant, MirVariantKind};
use crate::codegen::ir::mir::ty::structure::MirStruct;
use crate::library::codegen::generator::api_dart::spec_generator::base::*;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::utils::basic_code::dart_header_code::DartHeaderCode;
use itertools::Itertools;

const BACKTRACE_IDENT: &str = "backtrace";

impl<'a> EnumRefApiDartGenerator<'a> {
    pub(crate) fn generate_mode_complex(
        &self,
        src: &MirEnum,
        extra_body: &str,
        header: DartHeaderCode,
    ) -> Option<ApiDartGeneratedClass> {
        let variants = src
            .variants()
            .iter()
            .map(|variant| self.generate_mode_complex_variant(variant))
            .collect_vec()
            .join("\n");
        let name = &self.mir.ident.0.name;
        let sealed = if self.context.config.dart3 {
            "sealed"
        } else {
            ""
        };
        let maybe_implements_exception =
            generate_dart_maybe_implements_exception(self.mir.is_exception);

        Some(ApiDartGeneratedClass {
            namespace: src.name.namespace.clone(),
            class_name: name.clone(),
            code: format!(
                "@freezed
                {sealed} class {name} with _${name} {maybe_implements_exception} {{
                    const {name}._();

                    {variants}

                    {extra_body}
                }}",
            ),
            needs_freezed: true,
            header,
        })
    }

    fn generate_mode_complex_variant(&self, variant: &MirEnumVariant) -> String {
        let args = match &variant.kind {
            MirVariantKind::Value => "".to_owned(),
            MirVariantKind::Struct(st) => {
                if st.is_fields_named {
                    self.generate_variant_struct_named(st)
                } else {
                    self.generate_variant_struct_unnamed(st)
                }
            }
        };

        let implements_exception = self.generate_implements_exception(variant);

        format!(
            "{} {}const factory {}.{}({}) = {};",
            implements_exception,
            generate_dart_comments(&variant.comments),
            self.mir.ident.0.name,
            variant.name.dart_style(),
            args,
            variant.wrapper_name.rust_style(),
        )
    }

    fn generate_variant_struct_unnamed(&self, st: &MirStruct) -> String {
        let types = st
            .fields
            .iter()
            .map(|field| {
                // If no split, default values are not valid.
                let default = optional_boundary_index(&st.fields)
                    .is_some()
                    .then(|| {
                        generate_field_default(field, true, self.context.config.dart_enums_style)
                    })
                    .unwrap_or_default();
                let comments = generate_dart_comments(&field.comments);
                let type_str =
                    ApiDartGenerator::new(field.ty.clone(), self.context).dart_api_type();
                let name_str = field.name.dart_style();
                format!("{comments} {default} {type_str} {name_str},")
            })
            .collect_vec();

        if let Some(idx) = optional_boundary_index(&st.fields) {
            let before = &types[..idx];
            let after = &types[idx..];
            format!("{}[{}]", before.join(""), after.join(""))
        } else {
            types.join("")
        }
    }

    fn generate_variant_struct_named(&self, st: &MirStruct) -> String {
        let fields = st
            .fields
            .iter()
            .map(|field| {
                format!(
                    "{comments} {default} {required}{} {} ,",
                    ApiDartGenerator::new(field.ty.clone(), self.context).dart_api_type(),
                    field.name.dart_style(),
                    required = generate_field_required_modifier(field),
                    comments = generate_dart_comments(&field.comments),
                    default =
                        generate_field_default(field, true, self.context.config.dart_enums_style),
                )
            })
            .collect_vec();
        format!("{{ {} }}", fields.join(""))
    }

    fn generate_implements_exception(&self, variant: &MirEnumVariant) -> &str {
        let has_backtrace = matches!(&variant.kind,
            MirVariantKind::Struct(MirStruct {is_fields_named: true, fields, ..}) if fields.iter().any(|field| field.name.rust_style() == BACKTRACE_IDENT));
        if self.mir.is_exception && has_backtrace {
            "@Implements<FrbBacktracedException>()"
        } else {
            ""
        }
    }
}

fn optional_boundary_index(fields: &[MirField]) -> Option<usize> {
    fields
        .iter()
        .enumerate()
        .find(|(_, field)| field.is_optional())
        .and_then(|(idx, _)| {
            fields[idx..]
                .iter()
                .all(|field| field.is_optional())
                .then_some(idx)
        })
}
