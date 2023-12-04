use crate::codegen::ir::field::{IrField, IrFieldSettings};
use crate::codegen::ir::ident::IrIdent;
use crate::codegen::ir::namespace::NamespacedName;
use crate::codegen::ir::ty::structure::{IrStruct, IrStructIdent, IrTypeStructRef};
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::StructRef;
use crate::codegen::parser::attribute_parser::FrbAttributes;
use crate::codegen::parser::source_graph::modules::Struct;
use crate::codegen::parser::type_parser::enum_or_struct::{
    EnumOrStructParser, EnumOrStructParserInfo,
};
use crate::codegen::parser::type_parser::misc::parse_comments;
use crate::codegen::parser::type_parser::unencodable::SplayedSegment;
use crate::codegen::parser::type_parser::TypeParserWithContext;
use std::collections::HashMap;
use syn::{Field, Fields, FieldsNamed, FieldsUnnamed, ItemStruct, TypePath};

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path_data_struct(
        &mut self,
        type_path: &TypePath,
        splayed_segments: &[SplayedSegment],
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<IrType>> {
        EnumOrStructParserStruct(self).parse(type_path, splayed_segments, last_segment)
    }

    fn parse_struct(
        &mut self,
        src_struct: &Struct,
        name: NamespacedName,
        wrapper_name: Option<String>,
    ) -> anyhow::Result<Option<IrStruct>> {
        let (is_fields_named, struct_fields) = match &src_struct.0.src.fields {
            Fields::Named(FieldsNamed { named, .. }) => (true, named),
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => (false, unnamed),
            _ => return Ok(None),
        };

        let fields = struct_fields
            .iter()
            .enumerate()
            .map(|(idx, field)| self.parse_struct_field(idx, field))
            .collect::<anyhow::Result<Vec<_>>>()?;

        let comments = parse_comments(&src_struct.0.src.attrs);

        let attributes = FrbAttributes::parse(&src_struct.0.src.attrs)?;
        let dart_metadata = attributes.dart_metadata();

        Ok(Some(IrStruct {
            name,
            wrapper_name,
            fields,
            is_fields_named,
            dart_metadata,
            comments,
        }))
    }

    fn parse_struct_field(&mut self, idx: usize, field: &Field) -> anyhow::Result<IrField> {
        let field_name = field
            .ident
            .as_ref()
            .map_or(format!("field{idx}"), ToString::to_string);
        let field_type = self.parse_type(&field.ty)?;
        let attributes = FrbAttributes::parse(&field.attrs)?;
        Ok(IrField {
            name: IrIdent::new(field_name),
            ty: field_type,
            is_final: !attributes.non_final(),
            comments: parse_comments(&field.attrs),
            default: attributes.default_value(),
            settings: IrFieldSettings::default(),
        })
    }
}

struct EnumOrStructParserStruct<'a, 'b, 'c, 'd>(&'d mut TypeParserWithContext<'a, 'b, 'c>);

impl EnumOrStructParser<IrStructIdent, IrStruct, Struct, ItemStruct>
    for EnumOrStructParserStruct<'_, '_, '_, '_>
{
    fn parse_inner(
        &mut self,
        src_object: &Struct,
        name: NamespacedName,
        wrapper_name: Option<String>,
    ) -> anyhow::Result<Option<IrStruct>> {
        self.0.parse_struct(src_object, name, wrapper_name)
    }

    fn construct_output(&self, ident: IrStructIdent) -> anyhow::Result<IrType> {
        Ok(StructRef(IrTypeStructRef {
            ident,
            is_exception: false,
        }))
    }

    fn src_objects(&self) -> &HashMap<String, &Struct> {
        &self.0.inner.src_structs
    }

    fn parser_info(&mut self) -> &mut EnumOrStructParserInfo<IrStructIdent, IrStruct> {
        &mut self.0.inner.struct_parser_info
    }
}
