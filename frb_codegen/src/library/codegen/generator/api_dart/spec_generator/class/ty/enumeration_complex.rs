use crate::codegen::generator::api_dart::spec_generator::class::field::{
    generate_field_default, generate_field_default_for_constructor,
    generate_field_required_modifier,
};
use crate::codegen::generator::api_dart::spec_generator::class::ty::structure_non_freezed::{
    generate_equals, generate_hashcode, needs_deep_equality,
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

impl EnumRefApiDartGenerator<'_> {
    pub(crate) fn generate_mode_complex(
        &self,
        src: &MirEnum,
        extra_body: &str,
        header: DartHeaderCode,
    ) -> Option<ApiDartGeneratedClass> {
        let use_freezed = src
            .dart_enums_freezed
            .unwrap_or(self.context.config.dart_enums_freezed)
            || src.needs_json_serializable;
        let variants = src
            .variants()
            .iter()
            .map(|variant| self.generate_mode_complex_variant_factory(variant, use_freezed))
            .collect_vec()
            .join("\n");
        let variant_classes = if !use_freezed {
            self.generate_mode_complex_variant_classes(src)
        } else {
            Default::default()
        };
        let name = &self.mir.ident.0.name;
        let sealed = if self.context.config.dart3 {
            "sealed"
        } else {
            ""
        };
        let maybe_implements_exception =
            generate_dart_maybe_implements_exception(self.mir.is_exception);

        let json_serializable_extra_code =
            compute_json_serializable_extra_code(src.needs_json_serializable, name);

        let code = if use_freezed {
            format!(
                "@freezed
                {sealed} class {name} with _${name} {maybe_implements_exception} {{
                    const {name}._();

                    {variants}

                    {json_serializable_extra_code}

                    {extra_body}
                }}",
            )
        } else {
            format!(
                "{sealed} class {name} {maybe_implements_exception} {{
                    const {name}._();

                    {variants}

                    {native_extra_body}

                    {extra_body}
                }}

                {variant_classes}",
                native_extra_body = self.generate_native_extra_body(src),
            )
        };

        let header = if !use_freezed && enum_needs_deep_equality(src) {
            header
                + DartHeaderCode {
                    import: "import 'package:collection/collection.dart';\n".to_owned(),
                    ..Default::default()
                }
        } else {
            header
        };

        Some(ApiDartGeneratedClass {
            namespace: src.name.namespace.clone(),
            class_name: name.clone(),
            code,
            needs_freezed: use_freezed,
            needs_json_serializable: src.needs_json_serializable,
            header,
        })
    }

    fn generate_mode_complex_variant_factory(
        &self,
        variant: &MirEnumVariant,
        use_freezed: bool,
    ) -> String {
        let args = match &variant.kind {
            MirVariantKind::Value => "".to_owned(),
            MirVariantKind::Struct(st) => {
                if st.is_fields_named {
                    self.generate_variant_struct_named(st, use_freezed)
                } else {
                    self.generate_variant_struct_unnamed(st, use_freezed)
                }
            }
        };

        let implements_exception = if use_freezed {
            self.generate_freezed_implements_exception(variant)
        } else {
            ""
        };

        format!(
            "{} {}const factory {}.{}({}) = {};",
            implements_exception,
            generate_dart_comments(&variant.comments),
            self.mir.ident.0.name,
            variant.name.dart_style(),
            args,
            variant.wrapper_name.rust_style(true),
        )
    }

    fn generate_variant_struct_unnamed(&self, st: &MirStruct, freezed: bool) -> String {
        let types = st
            .fields
            .iter()
            .map(|field| {
                // If no split, default values are not valid.
                let default = if freezed && optional_boundary_index(&st.fields).is_some() {
                    generate_field_default(field, freezed, self.context.config.dart_enums_style)
                } else {
                    Default::default()
                };
                let comments = generate_dart_comments(&field.comments);
                let type_str =
                    ApiDartGenerator::new(field.ty.clone(), self.context).dart_api_type();
                let name_str = field.name.dart_style();
                if freezed {
                    format!("{comments} {default} {type_str} {name_str},")
                } else {
                    format!("{comments} {type_str} {name_str} {default},")
                }
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

    fn generate_variant_struct_named(&self, st: &MirStruct, freezed: bool) -> String {
        if st.fields.is_empty() {
            return "".to_owned();
        }

        let fields = st
            .fields
            .iter()
            .map(|field| {
                let comments = generate_dart_comments(&field.comments);
                let type_str =
                    ApiDartGenerator::new(field.ty.clone(), self.context).dart_api_type();
                let name = field.name.dart_style();
                let required = generate_field_required_modifier(field);
                if freezed {
                    let default =
                        generate_field_default(field, true, self.context.config.dart_enums_style);
                    format!("{comments} {default} {required}{type_str} {name} ,")
                } else {
                    format!("{comments} {required}{type_str} {name},")
                }
            })
            .collect_vec();
        format!("{{ {} }}", fields.join(""))
    }

    fn generate_mode_complex_variant_classes(&self, src: &MirEnum) -> String {
        src.variants()
            .iter()
            .map(|variant| self.generate_mode_complex_variant_class(variant))
            .collect_vec()
            .join("\n")
    }

    fn generate_native_extra_body(&self, src: &MirEnum) -> String {
        [
            self.generate_native_common_getters(src),
            self.generate_native_when_or_null(src),
        ]
        .into_iter()
        .filter(|part| !part.is_empty())
        .join("\n")
    }

    fn generate_native_when_or_null(&self, src: &MirEnum) -> String {
        let generic = "TResult";
        let callbacks = src
            .variants()
            .iter()
            .map(|variant| {
                format!(
                    "{}? {},",
                    self.generate_native_when_or_null_callback_type(variant, generic),
                    variant.name.dart_style(),
                )
            })
            .collect_vec()
            .join("\n");
        let branches = src
            .variants()
            .iter()
            .map(|variant| {
                let wrapper_name = variant.wrapper_name.rust_style(true);
                let variant_name = variant.name.dart_style();
                let args = self.generate_native_when_or_null_callback_args(variant);
                format!(
                    "if (self is {wrapper_name}) {{
                        return {variant_name}?.call({args});
                    }}"
                )
            })
            .collect_vec()
            .join("\n");

        format!(
            "{generic}? whenOrNull<{generic} extends Object?>({{
                {callbacks}
            }}) {{
                final self = this;
                {branches}
                return null;
            }}"
        )
    }

    fn generate_native_when_or_null_callback_type(
        &self,
        variant: &MirEnumVariant,
        generic: &str,
    ) -> String {
        let fields = variant.kind.fields();
        if fields.is_empty() {
            return format!("{generic} Function()");
        }

        let fields = fields
            .iter()
            .map(|field| {
                let required = if matches!(&variant.kind, MirVariantKind::Struct(st) if st.is_fields_named) {
                    "required "
                } else {
                    ""
                };
                format!(
                    "{required}{} {}",
                    ApiDartGenerator::new(field.ty.clone(), self.context).dart_api_type(),
                    field.name.dart_style(),
                )
            })
            .collect_vec()
            .join(",");

        if matches!(&variant.kind, MirVariantKind::Struct(st) if st.is_fields_named) {
            format!("{generic} Function({{ {fields} }})")
        } else {
            format!("{generic} Function({fields})")
        }
    }

    fn generate_native_when_or_null_callback_args(&self, variant: &MirEnumVariant) -> String {
        let fields = variant.kind.fields();
        if fields.is_empty() {
            return "".to_owned();
        }

        fields
            .iter()
            .map(|field| {
                let name = field.name.dart_style();
                if matches!(&variant.kind, MirVariantKind::Struct(st) if st.is_fields_named) {
                    format!("{name}: self.{name}")
                } else {
                    format!("self.{name}")
                }
            })
            .collect_vec()
            .join(",")
    }

    fn generate_native_common_getters(&self, src: &MirEnum) -> String {
        self.compute_native_common_getters(src)
            .into_iter()
            .map(|(field_name, type_str)| {
                let branches = src
                    .variants()
                    .iter()
                    .map(|variant| {
                        let wrapper_name = variant.wrapper_name.rust_style(true);
                        format!(
                            "if (self is {wrapper_name}) {{
                                return self.{field_name};
                            }}"
                        )
                    })
                    .collect_vec()
                    .join("\n");
                format!(
                    "{type_str} get {field_name} {{
                        final self = this;
                        {branches}
                        throw StateError('Unreachable enum variant');
                    }}"
                )
            })
            .collect_vec()
            .join("\n")
    }

    fn compute_native_common_getters(&self, src: &MirEnum) -> Vec<(String, String)> {
        let variants = src.variants();
        let Some(first_fields) = variants.first().and_then(|variant| match &variant.kind {
            MirVariantKind::Struct(MirStruct {
                is_fields_named: true,
                fields,
                ..
            }) => Some(fields),
            _ => None,
        }) else {
            return vec![];
        };

        first_fields
            .iter()
            .filter_map(|first_field| {
                let field_name = first_field.name.dart_style();
                let field_types = variants
                    .iter()
                    .map(|variant| self.find_named_variant_field_type(variant, &field_name))
                    .collect::<Option<Vec<_>>>()?;
                let type_str = field_types
                    .first()
                    .filter(|first| field_types.iter().all(|value| value == *first))
                    .cloned()
                    .unwrap_or_else(|| "Object?".to_owned());
                Some((field_name, type_str))
            })
            .collect_vec()
    }

    fn find_named_variant_field_type(
        &self,
        variant: &MirEnumVariant,
        field_name: &str,
    ) -> Option<String> {
        let MirVariantKind::Struct(MirStruct {
            is_fields_named: true,
            fields,
            ..
        }) = &variant.kind
        else {
            return None;
        };
        fields
            .iter()
            .find(|field| field.name.dart_style() == field_name)
            .map(|field| ApiDartGenerator::new(field.ty.clone(), self.context).dart_api_type())
    }

    fn generate_mode_complex_variant_class(&self, variant: &MirEnumVariant) -> String {
        let enum_name = &self.mir.ident.0.name;
        let wrapper_name = variant.wrapper_name.rust_style(true);
        let field_declarations = self.generate_variant_field_declarations(variant);
        let constructor_params = self.generate_variant_constructor_params(variant);
        let implements_exception = self.generate_native_implements_exception(variant);
        let fields = variant.kind.fields();
        let hashcode = generate_hashcode(&fields, true);
        let equals = generate_equals(&fields, &wrapper_name, true);

        format!(
            "class {wrapper_name} extends {enum_name} {implements_exception} {{
                {field_declarations}

                const {wrapper_name}({constructor_params}) : super._();

                {hashcode}

                {equals}
            }}"
        )
    }

    fn generate_variant_field_declarations(&self, variant: &MirEnumVariant) -> String {
        variant
            .kind
            .fields()
            .iter()
            .map(|field| {
                let comments = generate_dart_comments(&field.comments);
                let type_str =
                    ApiDartGenerator::new(field.ty.clone(), self.context).dart_api_type();
                let name_str = field.name.dart_style();
                format!("{comments}final {type_str} {name_str};")
            })
            .collect_vec()
            .join("\n")
    }

    fn generate_variant_constructor_params(&self, variant: &MirEnumVariant) -> String {
        match &variant.kind {
            MirVariantKind::Value => "".to_owned(),
            MirVariantKind::Struct(st) => {
                if st.is_fields_named {
                    self.generate_variant_constructor_params_named(st)
                } else {
                    self.generate_variant_constructor_params_unnamed(st)
                }
            }
        }
    }

    fn generate_variant_constructor_params_named(&self, st: &MirStruct) -> String {
        if st.fields.is_empty() {
            return "".to_owned();
        }

        let fields = st
            .fields
            .iter()
            .map(|field| {
                format!(
                    "{required}this.{name} {default},",
                    name = field.name.dart_style(),
                    required = generate_field_required_modifier(field),
                    default = generate_field_default_for_constructor(
                        field,
                        self.context.config.dart_enums_style
                    ),
                )
            })
            .collect_vec()
            .join("");

        format!("{{ {fields} }}")
    }

    fn generate_variant_constructor_params_unnamed(&self, st: &MirStruct) -> String {
        let fields = st
            .fields
            .iter()
            .map(|field| {
                let default = if optional_boundary_index(&st.fields).is_some() {
                    generate_field_default_for_constructor(
                        field,
                        self.context.config.dart_enums_style,
                    )
                } else {
                    Default::default()
                };
                format!("this.{} {default},", field.name.dart_style())
            })
            .collect_vec();

        if let Some(idx) = optional_boundary_index(&st.fields) {
            let before = &fields[..idx];
            let after = &fields[idx..];
            format!("{}[{}]", before.join(""), after.join(""))
        } else {
            fields.join("")
        }
    }

    fn generate_freezed_implements_exception(&self, variant: &MirEnumVariant) -> &str {
        let has_backtrace = matches!(&variant.kind,
            MirVariantKind::Struct(MirStruct {is_fields_named: true, fields, ..}) if fields.iter().any(|field| field.name.rust_style(true) == BACKTRACE_IDENT));
        if self.mir.is_exception && has_backtrace {
            "@Implements<FrbBacktracedException>()"
        } else {
            ""
        }
    }

    fn generate_native_implements_exception(&self, variant: &MirEnumVariant) -> &str {
        let has_backtrace = matches!(&variant.kind,
            MirVariantKind::Struct(MirStruct {is_fields_named: true, fields, ..}) if fields.iter().any(|field| field.name.rust_style(true) == BACKTRACE_IDENT));
        if self.mir.is_exception && has_backtrace {
            "implements FrbBacktracedException"
        } else {
            ""
        }
    }
}

fn enum_needs_deep_equality(src: &MirEnum) -> bool {
    src.variants()
        .iter()
        .flat_map(|variant| variant.kind.fields())
        .any(|field| needs_deep_equality(&field.ty))
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

pub(crate) fn compute_json_serializable_extra_code(
    needs_json_serializable: bool,
    name: &str,
) -> String {
    if needs_json_serializable {
        format!("factory {name}.fromJson(Map<String, dynamic> json) => _${name}FromJson(json);")
    } else {
        "".to_owned()
    }
}
