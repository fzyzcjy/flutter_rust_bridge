use crate::codegen::ir::hir::flat::struct_or_enum::HirFlatEnum;
use crate::codegen::ir::mir::field::{MirField, MirFieldSettings};
use crate::codegen::ir::mir::ident::MirIdent;
use crate::codegen::ir::mir::ty::boxed::MirTypeBoxed;
use crate::codegen::ir::mir::ty::delegate::{MirTypeDelegate, MirTypeDelegatePrimitiveEnum};
use crate::codegen::ir::mir::ty::enumeration::{
    MirEnum, MirEnumIdent, MirEnumMode, MirEnumVariant, MirTypeEnumRef, MirVariantKind,
};
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::ir::mir::ty::rust_auto_opaque_implicit::MirTypeRustAutoOpaqueImplicitReason;
use crate::codegen::ir::mir::ty::structure::MirStruct;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::ir::mir::ty::MirType::{Delegate, EnumRef};
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
use crate::codegen::parser::mir::parser::ty::enum_or_struct::{
    parse_struct_or_enum_should_ignore, EnumOrStructParser, EnumOrStructParserInfo,
};
use crate::codegen::parser::mir::parser::ty::generics::parse_generics_info;
use crate::codegen::parser::mir::parser::ty::type_substitution::{
    generate_concrete_name, has_concrete_generic_args, instantiate_enum, strip_path_generic_args,
};
use crate::codegen::parser::mir::parser::ty::misc::parse_comments;
use crate::codegen::parser::mir::parser::ty::structure::structure_compute_default_opaque;
use crate::codegen::parser::mir::parser::ty::unencodable::SplayedSegment;
use crate::codegen::parser::mir::parser::ty::{TypeParserParsingContext, TypeParserWithContext};
use crate::if_then_some;
use crate::utils::basic_code::general_code::GeneralDartCode;
use crate::utils::namespace::{Namespace, NamespacedName};
use std::collections::HashMap;
use syn::{Attribute, Field, Ident, ItemEnum, Type, Variant, Visibility};

/// Generate a name for a MirType to be used in auto-generated enum/struct names
/// This is used when instantiating generics without explicit type aliases
pub(crate) fn generate_type_name_for_auto_naming(ty: &MirType) -> String {
    match ty {
        MirType::Record(record) => {
            let mut parts = vec!["Record".to_string()];
            for val in record.values.iter() {
                parts.push(generate_type_name_for_auto_naming(val));
            }
            parts.join("_")
        }
        MirType::Primitive(p) => format!("{:?}", p),
        MirType::EnumRef(e) => e.ident.0.name.clone(),
        MirType::StructRef(s) => s.ident.0.name.clone(),
        MirType::Optional(opt) => format!("Optional_{}", generate_type_name_for_auto_naming(&opt.inner)),
        MirType::Boxed(boxed) => format!("Box_{}", generate_type_name_for_auto_naming(&boxed.inner)),
        MirType::GeneralList(list) => format!("List_{}", generate_type_name_for_auto_naming(&list.inner)),
        MirType::Delegate(delegate) => {
            match delegate {
                MirTypeDelegate::String => "String".to_string(),
                MirTypeDelegate::Map(map) => {
                    format!("Map_{}_{}",
                        generate_type_name_for_auto_naming(&map.key),
                        generate_type_name_for_auto_naming(&map.value))
                }
                MirTypeDelegate::StreamSink(sink) => {
                    format!("StreamSink_{}", generate_type_name_for_auto_naming(&sink.inner_ok))
                }
                _ => format!("Delegate_{:?}", delegate),
            }
        }
        _ => format!("Type_{}", format!("{:?}", ty).replace(" ", "_").replace("::", "_")),
    }
}

impl TypeParserWithContext<'_, '_, '_> {
    pub(crate) fn parse_type_path_data_enum_with_alias(
        &mut self,
        path: &syn::Path,
        last_segment: &SplayedSegment,
        alias_name: Option<&str>,
    ) -> anyhow::Result<Option<MirType>> {
        let (name, generic_args) = last_segment;
        let namespace = self.parse_enum_namespace(name);

        if !generic_args.is_empty()
            && !has_concrete_generic_args(generic_args, &self.context.current_generic_params)
        {
            let template_path = strip_path_generic_args(path);
            let empty_args: &[syn::Type] = &[];
            return EnumOrStructParserEnum(self).parse(&template_path, &(name, empty_args), None);
        }

        if !generic_args.is_empty() {
            if let Some(namespace) = namespace.clone() {
                let template_name = NamespacedName::new(namespace.clone(), name.to_string());
                let template_ident: MirEnumIdent = template_name.clone().into();

                // Ensure template is parsed on demand
                if !self.inner.enum_parser_info.generic_templates.contains_key(&template_ident) {
                    let template_path = strip_path_generic_args(path);
                    let empty_args: &[syn::Type] = &[];
                    let _ = EnumOrStructParserEnum(self)
                        .parse(&template_path, &(name, empty_args), None)?;
                }

                if let Some(template) = self
                    .inner
                    .enum_parser_info
                    .generic_templates
                    .get(&template_ident)
                    .cloned()
                {
                    if template.generic_params.len() != generic_args.len() {
                        let type_path = syn::TypePath { qself: None, path: path.clone() };
                        return Ok(Some(self.parse_type_rust_auto_opaque_implicit(
                            Some(namespace.clone()),
                            &syn::Type::Path(type_path),
                            None,
                            None,
                        )?));
                    }

                    let type_args: Vec<MirType> = generic_args
                        .iter()
                        .map(|arg_ty| self.parse_type(arg_ty))
                        .collect::<anyhow::Result<Vec<_>>>()?;

                    let concrete_name =
                        generate_concrete_name(namespace, name, alias_name, &type_args);
                    let concrete_ident: MirEnumIdent = concrete_name.clone().into();

                    if self.inner.enum_parser_info.object_pool.contains_key(&concrete_ident) {
                        return Ok(Some(MirType::EnumRef(MirTypeEnumRef {
                            ident: concrete_ident,
                            is_exception: false,
                        })));
                    }

                    let is_from_type_alias =
                        alias_name.is_some() || self.context.is_within_type_alias;
                    let concrete_enum =
                        instantiate_enum(&template, concrete_name, &type_args, is_from_type_alias)?;

                    self.inner
                        .enum_parser_info
                        .object_pool
                        .insert(concrete_ident.clone(), concrete_enum);

                    return Ok(Some(MirType::EnumRef(MirTypeEnumRef {
                        ident: concrete_ident,
                        is_exception: false,
                    })));
                }
            }
        }

        EnumOrStructParserEnum(self).parse(path, last_segment, None)
    }

    pub(crate) fn parse_enum_namespace(&mut self, name: &str) -> Option<Namespace> {
        EnumOrStructParserEnum(self).parse_namespace(name)
    }

    fn parse_enum(
        &mut self,
        src_enum: &HirFlatEnum,
        name: NamespacedName,
        wrapper_name: Option<String>,
    ) -> anyhow::Result<MirEnum> {
        let comments = parse_comments(&src_enum.src.attrs);

        // Extract generic parameters if this is a generic enum
        let generics_info = parse_generics_info(&src_enum.src.generics);
        let (generic_params, is_generic_template) = match generics_info {
            crate::codegen::parser::mir::parser::ty::generics::GenericsInfo::Generic { params } => {
                (params.clone(), true)
            }
            _ => (vec![], false),
        };

        let raw_variants = src_enum
            .src
            .variants
            .iter()
            .map(|variant| self.parse_variant(src_enum, variant, &generic_params))
            .collect::<anyhow::Result<Vec<_>>>()?;

        let mode = compute_enum_mode(&raw_variants);
        let variants = maybe_field_wrap_box(raw_variants, mode);
        let ignore = parse_struct_or_enum_should_ignore(
            src_enum,
            &name.namespace.crate_name(),
            self.context,
        );

        let attributes = FrbAttributes::parse(&src_enum.src.attrs)?;

        Ok(MirEnum {
            name,
            wrapper_name,
            comments,
            variants,
            mode,
            ignore,
            needs_json_serializable: attributes.json_serializable(),
            generic_params,
            is_generic_template,
        })
    }

    fn parse_variant(
        &mut self,
        src_enum: &HirFlatEnum,
        variant: &Variant,
        generic_params: &[String],
    ) -> anyhow::Result<MirEnumVariant> {
        let variant_name = MirIdent::new(variant.ident.to_string(), None);
        Ok(MirEnumVariant {
            name: variant_name.clone(),
            wrapper_name: MirIdent::new(format!("{}_{}", src_enum.name.name, variant.ident), None),
            comments: parse_comments(&variant.attrs),
            kind: match variant.fields.iter().next() {
                None => MirVariantKind::Value,
                Some(Field {
                    attrs,
                    ident: field_ident,
                    ..
                }) => self.parse_variant_kind_struct(
                    src_enum,
                    variant,
                    attrs,
                    field_ident,
                    &variant_name,
                    generic_params,
                )?,
            },
        })
    }

    fn parse_variant_kind_struct(
        &mut self,
        src_enum: &HirFlatEnum,
        variant: &Variant,
        attrs: &[Attribute],
        field_ident: &Option<Ident>,
        variant_name: &MirIdent,
        generic_params: &[String],
    ) -> anyhow::Result<MirVariantKind> {
        let attributes = FrbAttributes::parse(attrs)?;
        Ok(MirVariantKind::Struct(MirStruct {
            name: compute_enum_variant_kind_struct_name(&src_enum.name, variant_name),
            wrapper_name: None,
            is_fields_named: field_ident.is_some(),
            dart_metadata_raw: attributes.dart_metadata(),
            ignore: attributes.ignore(),
            needs_json_serializable: attributes.json_serializable(),
            generate_hash: true,
            generate_eq: true,
            ui_state: attributes.ui_state(),
            comments: parse_comments(attrs),
            fields: variant
                .fields
                .iter()
                .enumerate()
                .map(|(idx, field)| {
                    Ok(MirField {
                        name: MirIdent::new(
                            field
                                .ident
                                .as_ref()
                                .map(ToString::to_string)
                                .unwrap_or_else(|| format!("field{idx}")),
                            None,
                        ),
                        ty: self.parse_type_with_context(&field.ty, |c| {
                            c.with_struct_or_enum_attributes(attributes.clone())
                                .with_generic_params(generic_params.to_vec())
                        })?,
                        is_final: true,
                        is_rust_public: Some(matches!(field.vis, Visibility::Public(_))),
                        comments: parse_comments(&field.attrs),
                        default: FrbAttributes::parse(&field.attrs)?.default_value(),
                        settings: MirFieldSettings {
                            is_in_mirrored_enum: src_enum.mirror,
                            ..Default::default()
                        },
                    })
                })
                .collect::<anyhow::Result<Vec<_>>>()?,
            generic_params: vec![],
            is_generic_template: false,
        }))
    }
}

pub(crate) fn compute_enum_variant_kind_struct_name(
    enum_name: &NamespacedName,
    variant_name: &MirIdent,
) -> NamespacedName {
    let variant_namespace = enum_name.namespace.join(&enum_name.name);
    NamespacedName::new(variant_namespace, variant_name.rust_style(true).to_owned())
}

struct EnumOrStructParserEnum<'a, 'b, 'c, 'd>(&'d mut TypeParserWithContext<'a, 'b, 'c>);

impl EnumOrStructParser<MirEnumIdent, MirEnum, ItemEnum>
    for EnumOrStructParserEnum<'_, '_, '_, '_>
{
    fn parse_inner_impl(
        &mut self,
        src_object: &HirFlatEnum,
        name: NamespacedName,
        wrapper_name: Option<String>,
    ) -> anyhow::Result<MirEnum> {
        self.0.parse_enum(src_object, name, wrapper_name)
    }

    fn construct_output(&self, ident: MirEnumIdent) -> anyhow::Result<MirType> {
        let enum_ref = MirTypeEnumRef {
            ident: ident.clone(),
            is_exception: false,
        };
        let enu = self.0.inner.enum_parser_info.object_pool.get(&ident);

        Ok(
            if enu.map(|e| e.mode == MirEnumMode::Complex).unwrap_or(true) {
                EnumRef(enum_ref)
            } else {
                Delegate(MirTypeDelegate::PrimitiveEnum(
                    MirTypeDelegatePrimitiveEnum {
                        mir: enum_ref,
                        // TODO(Desdaemon): Parse #[repr] from enum
                        repr: MirTypePrimitive::I32,
                    },
                ))
            },
        )
    }

    fn src_objects(&self) -> &HashMap<String, &HirFlatEnum> {
        &self.0.inner.src_enums
    }

    fn parser_info(&mut self) -> &mut EnumOrStructParserInfo<MirEnumIdent, MirEnum> {
        &mut self.0.inner.enum_parser_info
    }

    fn dart_code_of_type(&mut self) -> &mut HashMap<String, GeneralDartCode> {
        &mut self.0.inner.dart_code_of_type
    }

    fn parse_type_rust_auto_opaque_implicit(
        &mut self,
        namespace: Option<Namespace>,
        ty: &Type,
        reason: Option<MirTypeRustAutoOpaqueImplicitReason>,
        override_ignore: Option<bool>,
    ) -> anyhow::Result<MirType> {
        self.0
            .parse_type_rust_auto_opaque_implicit(namespace, ty, reason, override_ignore)
    }

    fn context(&self) -> &TypeParserParsingContext {
        self.0.context
    }

    fn compute_default_opaque(obj: &MirEnum) -> bool {
        obj.variants
            .iter()
            .filter_map(|variant| if_then_some!(let MirVariantKind::Struct(s) = &variant.kind, s))
            .any(|ty| structure_compute_default_opaque(ty, &obj.name.namespace.crate_name()))
    }

    fn is_generic_template(obj: &MirEnum) -> bool {
        obj.is_generic_template
    }
}

fn maybe_field_wrap_box(
    mut variants: Vec<MirEnumVariant>,
    mode: MirEnumMode,
) -> Vec<MirEnumVariant> {
    if mode == MirEnumMode::Complex {
        for variant in &mut variants {
            if let MirVariantKind::Struct(st) = &mut variant.kind {
                for field in &mut st.fields {
                    mir_type_wrap_box(&mut field.ty);
                }
            }
        }
    }

    variants
}

fn mir_type_wrap_box(ty: &mut MirType) {
    if ty.is_struct_or_enum_or_record() {
        *ty = MirType::Boxed(MirTypeBoxed {
            exist_in_real_api: false,
            inner: Box::new(ty.clone()),
        });
    }
}

fn compute_enum_mode(variants: &[MirEnumVariant]) -> MirEnumMode {
    if variants
        .iter()
        .any(|variant| !matches!(variant.kind, MirVariantKind::Value))
    {
        MirEnumMode::Complex
    } else {
        MirEnumMode::Simple
    }
}
