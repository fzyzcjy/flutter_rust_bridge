use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::general_list::IrTypeGeneralList;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::primitive_list::IrTypePrimitiveList;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{Delegate, GeneralList, Optional, Primitive, PrimitiveList};
use crate::codegen::parser::type_parser::unencodable::ArgsRefs::Generic;
use crate::codegen::parser::type_parser::unencodable::SplayedSegment;
use crate::codegen::parser::type_parser::TypeParserWithContext;
use anyhow::bail;
use quote::ToTokens;
use syn::TypePath;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path_data_vec(
        &mut self,
        type_path: &TypePath,
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<IrType>> {
        Ok(Some(match last_segment {
            ("Vec", Some(Generic([Delegate(IrTypeDelegate::String)]))) => {
                Delegate(IrTypeDelegate::StringList)
            }

            ("Vec", Some(Generic([Delegate(IrTypeDelegate::Uuid)]))) => {
                Delegate(IrTypeDelegate::Uuids)
            }

            ("Vec", Some(Generic([Primitive(primitive)]))) => {
                // Since Dart doesn't have a boolean primitive list like `Uint8List`,
                // we need to convert `Vec<bool>` to a boolean general list in order to achieve the binding.
                if primitive == &IrTypePrimitive::Bool {
                    GeneralList(IrTypeGeneralList {
                        inner: Box::new(IrType::Primitive(IrTypePrimitive::Bool)),
                    })
                } else {
                    PrimitiveList(IrTypePrimitiveList {
                        primitive: primitive.clone(),
                    })
                }
            }

            ("Vec", Some(Generic([Delegate(IrTypeDelegate::Time(time))]))) => {
                Delegate(IrTypeDelegate::TimeList(*time))
            }

            ("Vec", Some(Generic([element]))) => GeneralList(IrTypeGeneralList {
                inner: Box::new(element.clone()),
            }),

            _ => return Ok(None),
        }))
    }
}
