use crate::codegen::ir::field::{IrField, IrFieldSettings};
use crate::codegen::ir::ident::IrIdent;
use crate::codegen::ir::namespace::NamespacedName;
use crate::codegen::ir::ty::boxed::IrTypeBoxed;
use crate::codegen::ir::ty::delegate::{IrTypeDelegate, IrTypeDelegatePrimitiveEnum};
use crate::codegen::ir::ty::enumeration::{
    IrEnum, IrEnumIdent, IrEnumMode, IrTypeEnumRef, IrVariant, IrVariantKind,
};
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::structure::IrStruct;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{Delegate, EnumRef};
use crate::codegen::parser::attribute_parser::FrbAttributes;
use crate::codegen::parser::source_graph::modules::Enum;
use crate::codegen::parser::type_parser::enum_or_struct::{
    EnumOrStructParser, EnumOrStructParserInfo,
};
use crate::codegen::parser::type_parser::misc::parse_comments;
use crate::codegen::parser::type_parser::structure::compute_name_and_wrapper_name;
use crate::codegen::parser::type_parser::unencodable::SplayedSegment;
use crate::codegen::parser::type_parser::TypeParser;
use std::collections::HashMap;
use syn::{Attribute, Field, Ident, TypePath, Variant};

impl<'a> TypeParser<'a> {
    pub(crate) fn parse_type_path_data_enum(
        &mut self,
        type_path: &TypePath,
        splayed_segments: &[SplayedSegment],
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<IrType>> {
        EnumOrStructParserEnum(&mut self).parse(type_path, splayed_segments, last_segment)
    }

    fn parse_enum(&mut self, src_enum: &Enum) -> anyhow::Result<IrEnum> {
        let (name, wrapper_name) =
            compute_name_and_wrapper_name(&src_enum.0.ident, src_enum.0.mirror);

        let path = src_enum.0.path.clone();
        let comments = parse_comments(&src_enum.0.src.attrs);
        let raw_variants = src_enum
            .0
            .src
            .variants
            .iter()
            .map(|variant| self.parse_variant(src_enum, &variant))
            .collect::<anyhow::Result<Vec<_>>>()?;

        let mode = compute_enum_mode(&raw_variants);
        let variants = maybe_field_wrap_box(raw_variants, mode);

        Ok(IrEnum {
            name,
            wrapper_name,
            path,
            comments,
            variants,
            mode,
        })
    }

    fn parse_variant(&mut self, src_enum: &Enum, variant: &&Variant) -> anyhow::Result<IrVariant> {
        Ok(IrVariant {
            name: IrIdent::new(variant.ident.to_string()),
            wrapper_name: IrIdent::new(format!("{}_{}", src_enum.0.ident, variant.ident)),
            comments: parse_comments(&variant.attrs),
            kind: match variant.fields.iter().next() {
                None => IrVariantKind::Value,
                Some(Field {
                    attrs,
                    ident: field_ident,
                    ..
                }) => self.parse_variant_kind_struct(src_enum, variant, attrs, field_ident)?,
            },
        })
    }

    fn parse_variant_kind_struct(
        &mut self,
        src_enum: &Enum,
        variant: &&Variant,
        attrs: &Vec<Attribute>,
        field_ident: &Option<Ident>,
    ) -> anyhow::Result<IrVariantKind> {
        let variant_ident = variant.ident.to_string();
        Ok(IrVariantKind::Struct(IrStruct {
            name: NamespacedName::new(TODO, variant_ident),
            wrapper_name: None,
            path: None,
            is_fields_named: field_ident.is_some(),
            dart_metadata: FrbAttributes::parse(attrs)?.dart_metadata(),
            comments: parse_comments(attrs),
            fields: variant
                .fields
                .iter()
                .enumerate()
                .map(|(idx, field)| {
                    Ok(IrField {
                        name: IrIdent::new(
                            field
                                .ident
                                .as_ref()
                                .map(ToString::to_string)
                                .unwrap_or_else(|| format!("field{idx}")),
                        ),
                        ty: self.parse_type(&field.ty)?,
                        is_final: true,
                        comments: parse_comments(&field.attrs),
                        default: FrbAttributes::parse(&field.attrs)?.default_value(),
                        settings: IrFieldSettings {
                            is_in_mirrored_enum: src_enum.0.mirror,
                        },
                    })
                })
                .collect::<anyhow::Result<Vec<_>>>()?,
        }))
    }
}

struct EnumOrStructParserEnum<'a>(&'a mut TypeParser<'a>);

impl<'a> EnumOrStructParser<IrEnumIdent, IrEnum, Enum> for EnumOrStructParserEnum<'a> {
    fn parse_inner(&mut self, src_object: &Enum) -> anyhow::Result<Option<IrEnum>> {
        Ok(Some(self.0.parse_enum(src_object)?))
    }

    fn construct_output(&self, ident: IrEnumIdent) -> anyhow::Result<IrType> {
        let enum_ref = IrTypeEnumRef {
            ident: ident.clone(),
            is_exception: false,
        };
        let enu = self.0.enum_parser_info.get(&ident);

        return Ok(
            if enu.map(|e| e.mode == IrEnumMode::Complex).unwrap_or(true) {
                EnumRef(enum_ref)
            } else {
                Delegate(IrTypeDelegate::PrimitiveEnum(IrTypeDelegatePrimitiveEnum {
                    ir: enum_ref,
                    // TODO(Desdaemon): Parse #[repr] from enum
                    repr: IrTypePrimitive::I32,
                }))
            },
        );
    }

    fn src_objects(&self) -> &HashMap<String, &Enum> {
        &self.0.src_enums
    }

    fn parser_info(&mut self) -> &mut EnumOrStructParserInfo<IrEnumIdent, IrEnum> {
        &mut self.0.enum_parser_info
    }
}

fn maybe_field_wrap_box(mut variants: Vec<IrVariant>, mode: IrEnumMode) -> Vec<IrVariant> {
    if mode == IrEnumMode::Complex {
        for variant in &mut variants {
            if let IrVariantKind::Struct(st) = &mut variant.kind {
                for field in &mut st.fields {
                    ir_type_wrap_box(&mut field.ty);
                }
            }
        }
    }

    variants
}

fn ir_type_wrap_box(ty: &mut IrType) {
    if ty.is_struct_or_enum_or_record() {
        *ty = IrType::Boxed(IrTypeBoxed {
            exist_in_real_api: false,
            inner: Box::new(ty.clone()),
        });
    }
}

fn compute_enum_mode(variants: &[IrVariant]) -> IrEnumMode {
    if variants
        .iter()
        .any(|variant| !matches!(variant.kind, IrVariantKind::Value))
    {
        IrEnumMode::Complex
    } else {
        IrEnumMode::Simple
    }
}
