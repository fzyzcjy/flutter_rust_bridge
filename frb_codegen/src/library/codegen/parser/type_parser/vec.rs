use crate::codegen::ir::ty::general_list::{ir_list, IrTypeGeneralList};
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{GeneralList, Primitive};
use crate::codegen::parser::type_parser::unencodable::ArgsRefs::Generic;
use crate::codegen::parser::type_parser::unencodable::SplayedSegment;
use crate::codegen::parser::type_parser::TypeParserWithContext;
use syn::TypePath;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path_data_vec(
        &mut self,
        _type_path: &TypePath,
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<IrType>> {
        Ok(Some(match last_segment {
            // ("Vec", Some(Generic([Delegate(IrTypeDelegate::String)]))) => {
            //     Delegate(IrTypeDelegate::StringList)
            // }
            //
            // ("Vec", Some(Generic([Delegate(IrTypeDelegate::Uuid)]))) => {
            //     Delegate(IrTypeDelegate::Uuids)
            // }
            ("Vec", Some(Generic([Primitive(primitive)]))) => {
                ir_list(IrType::Primitive(primitive.to_owned()))
            }

            // ("Vec", Some(Generic([Delegate(IrTypeDelegate::Time(time))]))) => {
            //     Delegate(IrTypeDelegate::TimeList(*time))
            // }
            ("Vec", Some(Generic([element]))) => ir_list(element.to_owned()),

            _ => return Ok(None),
        }))
    }
}
