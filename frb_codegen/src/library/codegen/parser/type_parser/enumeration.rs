use crate::codegen::ir::comment::IrComment;
use crate::codegen::ir::default::IrDefaultValue;
use crate::codegen::ir::field::{IrField, IrFieldSettings};
use crate::codegen::ir::ident::IrIdent;
use crate::codegen::ir::ty::boxed::IrTypeBoxed;
use crate::codegen::ir::ty::enumeration::{IrEnum, IrVariant, IrVariantKind};
use crate::codegen::ir::ty::structure::IrStruct;
use crate::codegen::ir::ty::IrType;
use crate::codegen::parser::attribute_parser::FrbAttributes;
use crate::codegen::parser::type_parser::misc::parse_comments;
use crate::codegen::parser::type_parser::TypeParser;
use anyhow::anyhow;
use syn::Field;

impl<'a> TypeParser<'a> {
    fn parse_enum(&mut self, ident_string: &str) -> anyhow::Result<IrEnum> {
        let src_enum = self.src_enums[ident_string];
        let name = src_enum.0.ident.to_string();
        let wrapper_name = if src_enum.0.mirror {
            Some(format!("mirror_{name}"))
        } else {
            None
        };

        let path = src_enum.0.path.clone();
        let comments = parse_comments(&src_enum.0.src.attrs);
        let raw_variants = src_enum
            .0
            .src
            .variants
            .iter()
            .map(|variant| {
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
                        }) => {
                            let variant_ident = variant.ident.to_string();
                            IrVariantKind::Struct(IrStruct {
                                name: variant_ident,
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
                                            ty: self.parse_type(&field.ty),
                                            is_final: true,
                                            comments: parse_comments(&field.attrs),
                                            default: FrbAttributes::parse(&field.attrs)?
                                                .default_value(),
                                            settings: IrFieldSettings {
                                                is_in_mirrored_enum: src_enum.0.mirror,
                                            },
                                        })
                                    })
                                    .collect::<anyhow::Result<Vec<_>>>()?,
                            })
                        }
                    },
                })
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        let is_struct = compute_is_struct(&raw_variants);
        let variants = maybe_field_wrap_box(raw_variants, is_struct);

        Ok(IrEnum {
            name,
            wrapper_name,
            path,
            comments,
            variants,
            is_struct,
        })
    }
}

fn maybe_field_wrap_box(mut variants: Vec<IrVariant>, is_struct: bool) -> Vec<IrVariant> {
    if is_struct {
        for variant in &mut variants {
            if let IrVariantKind::Struct(st) = &mut variant.kind {
                for field in &mut st.fields {
                    wrap_box(&mut field.ty);
                }
            }
        }
    }

    variants
}

fn wrap_box(ty: &mut IrType) {
    if ty.is_struct_or_enum_or_record() {
        *ty = IrType::Boxed(IrTypeBoxed {
            exist_in_real_api: false,
            inner: Box::new(ty.clone()),
        });
    }
}

fn compute_is_struct(variants: &[IrVariant]) -> bool {
    variants
        .iter()
        .any(|variant| !matches!(variant.kind, IrVariantKind::Value))
}
