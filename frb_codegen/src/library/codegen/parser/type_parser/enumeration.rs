use crate::codegen::ir::default::IrDefaultValue;
use crate::codegen::ir::field::{IrField, IrFieldSettings};
use crate::codegen::ir::ident::IrIdent;
use crate::codegen::ir::ty::enumeration::{IrEnum, IrVariant, IrVariantKind};
use crate::codegen::ir::ty::structure::IrStruct;
use crate::codegen::parser::type_parser::misc::parse_comments;
use crate::codegen::parser::type_parser::TypeParser;
use syn::Field;

impl<'a> TypeParser<'a> {
    fn parse_enum(&mut self, ident_string: &str) -> IrEnum {
        let src_enum = self.src_enums[ident_string];
        let name = src_enum.ident.to_string();
        let wrapper_name = if src_enum.mirror {
            Some(format!("mirror_{name}"))
        } else {
            None
        };

        let path = src_enum.path.clone();
        let comments = parse_comments(&src_enum.src.attrs);
        let variants = src_enum
            .src
            .variants
            .iter()
            .map(|variant| IrVariant {
                name: IrIdent::new(variant.ident.to_string()),
                wrapper_name: IrIdent::new(format!("{}_{}", src_enum.ident, variant.ident)),
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
                            dart_metadata: parse_metadata(attrs),
                            comments: parse_comments(attrs),
                            fields: variant
                                .fields
                                .iter()
                                .enumerate()
                                .map(|(idx, field)| IrField {
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
                                    default: IrDefaultValue::extract(&field.attrs),
                                    settings: IrFieldSettings {
                                        is_in_mirrored_enum: src_enum.mirror,
                                    },
                                })
                                .collect(),
                        })
                    }
                },
            })
            .collect();
        IrEnum::new(name, wrapper_name, path, comments, variants, false)
    }
}
