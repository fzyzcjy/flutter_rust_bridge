use crate::codegen::ir::default::IrDefaultValue;
use crate::codegen::ir::field::{IrField, IrFieldSettings};
use crate::codegen::ir::ident::IrIdent;
use crate::codegen::ir::ty::structure::IrStruct;
use crate::codegen::parser::attribute_parser::FrbAttributes;
use crate::codegen::parser::type_parser::TypeParser;
use itertools::Itertools;
use syn::{Field, Fields, FieldsNamed, FieldsUnnamed};

impl<'a> TypeParser<'a> {
    fn parse_struct(&mut self, ident_string: &str) -> Option<IrStruct> {
        let src_struct = self.src_structs[ident_string];

        let (is_fields_named, struct_fields) = match &src_struct.src.fields {
            Fields::Named(FieldsNamed { named, .. }) => (true, named),
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => (false, unnamed),
            _ => return None,
        };

        let fields = struct_fields
            .iter()
            .map(|field| self.parse_struct_field(field))
            .collect_vec();

        let name = src_struct.ident.to_string();
        let wrapper_name = if src_struct.mirror {
            Some(format!("mirror_{name}"))
        } else {
            None
        };

        let path = Some(src_struct.path.clone());
        let attributes = FrbAttributes::parse(src_struct.src.attrs)?;
        let comments = extract_comments(&src_struct.src.attrs);

        Some(IrStruct {
            name,
            wrapper_name,
            path,
            fields,
            is_fields_named,
            dart_metadata,
            comments,
        })
    }

    fn parse_struct_field(&mut self, field: &Field) -> IrField {
        let field_name = field
            .ident
            .as_ref()
            .map_or(format!("field{idx}"), ToString::to_string);
        let field_type = self.parse_type(&field.ty);
        IrField {
            name: IrIdent::new(field_name),
            ty: field_type,
            // TODO use `metadata.non_final()`
            is_final: !markers::has_non_final(&field.attrs),
            comments: extract_comments(&field.attrs),
            default: IrDefaultValue::extract(&field.attrs),
            settings: IrFieldSettings::default(),
        }
    }
}
