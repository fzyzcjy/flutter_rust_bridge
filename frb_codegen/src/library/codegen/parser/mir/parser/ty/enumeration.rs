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
use crate::codegen::parser::mir::parser::ty::misc::parse_comments;
use crate::codegen::parser::mir::parser::ty::structure::structure_compute_default_opaque;
use crate::codegen::parser::mir::parser::ty::unencodable::SplayedSegment;
use crate::codegen::parser::mir::parser::ty::{TypeParserParsingContext, TypeParserWithContext};
use crate::if_then_some;
use crate::utils::basic_code::general_code::GeneralDartCode;
use crate::utils::namespace::{Namespace, NamespacedName};
use std::collections::HashMap;
use syn::{Attribute, Field, Ident, ItemEnum, Type, Variant, Visibility};

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path_data_enum(
        &mut self,
        path: &syn::Path,
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<MirType>> {
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
        let raw_variants = src_enum
            .src
            .variants
            .iter()
            .map(|variant| self.parse_variant(src_enum, variant))
            .collect::<anyhow::Result<Vec<_>>>()?;

        let mode = compute_enum_mode(&raw_variants);
        let variants = maybe_field_wrap_box(raw_variants, mode);
        let ignore = parse_struct_or_enum_should_ignore(
            src_enum,
            &name.namespace.crate_name(),
            self.context,
        );

        Ok(MirEnum {
            name,
            wrapper_name,
            comments,
            variants,
            mode,
            ignore,
        })
    }

    fn parse_variant(
        &mut self,
        src_enum: &HirFlatEnum,
        variant: &Variant,
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
    ) -> anyhow::Result<MirVariantKind> {
        let attributes = FrbAttributes::parse(attrs)?;
        Ok(MirVariantKind::Struct(MirStruct {
            name: compute_enum_variant_kind_struct_name(&src_enum.name, variant_name),
            wrapper_name: None,
            is_fields_named: field_ident.is_some(),
            dart_metadata: attributes.dart_metadata(),
            ignore: attributes.ignore(),
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
                        })?,
                        is_final: true,
                        is_rust_public: Some(matches!(field.vis, Visibility::Public(_))),
                        comments: parse_comments(&field.attrs),
                        default: FrbAttributes::parse(&field.attrs)?.default_value(),
                        settings: MirFieldSettings {
                            is_in_mirrored_enum: src_enum.mirror,
                        },
                    })
                })
                .collect::<anyhow::Result<Vec<_>>>()?,
        }))
    }
}

pub(crate) fn compute_enum_variant_kind_struct_name(
    enum_name: &NamespacedName,
    variant_name: &MirIdent,
) -> NamespacedName {
    let variant_namespace = enum_name.namespace.join(&enum_name.name);
    NamespacedName::new(variant_namespace, variant_name.rust_style().to_owned())
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
