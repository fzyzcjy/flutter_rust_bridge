use crate::codegen::hir::hierarchical::struct_or_enum::HirStruct;
use crate::codegen::mir::field::{IrField, IrFieldSettings};
use crate::codegen::mir::ident::IrIdent;
use crate::codegen::mir::namespace::{Namespace, NamespacedName};
use crate::codegen::mir::ty::structure::{IrStruct, IrStructIdent, IrTypeStructRef};
use crate::codegen::mir::ty::IrType;
use crate::codegen::mir::ty::IrType::StructRef;
use crate::codegen::parser::attribute_parser::FrbAttributes;
use crate::codegen::parser::type_parser::enum_or_struct::{
    EnumOrStructParser, EnumOrStructParserInfo,
};
use crate::codegen::parser::type_parser::misc::parse_comments;
use crate::codegen::parser::type_parser::unencodable::SplayedSegment;
use crate::codegen::parser::type_parser::TypeParserWithContext;
use anyhow::bail;
use std::collections::HashMap;
use syn::{Field, Fields, FieldsNamed, FieldsUnnamed, ItemStruct, Type, Visibility};

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path_data_struct(
        &mut self,
        last_segment: &SplayedSegment,
        override_opaque: Option<bool>,
    ) -> anyhow::Result<Option<IrType>> {
        EnumOrStructParserStruct(self).parse(last_segment, override_opaque)
    }

    fn parse_struct(
        &mut self,
        src_struct: &HirStruct,
        name: NamespacedName,
        wrapper_name: Option<String>,
    ) -> anyhow::Result<IrStruct> {
        let (is_fields_named, struct_fields) = match &src_struct.0.src.fields {
            Fields::Named(FieldsNamed { named, .. }) => (true, named),
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => (false, unnamed),
            // This will stop the whole generator and tell the users, so we do not care about testing it
            // frb-coverage:ignore-start
            Fields::Unit => bail!("struct with unit fields are not supported yet, what about using `struct {} {{}}` instead", src_struct.0.ident),
            // frb-coverage:ignore-end
        };

        let fields = struct_fields
            .iter()
            .enumerate()
            .map(|(idx, field)| self.parse_struct_field(idx, field))
            .collect::<anyhow::Result<Vec<_>>>()?;

        let comments = parse_comments(&src_struct.0.src.attrs);

        let attributes = FrbAttributes::parse(&src_struct.0.src.attrs)?;
        let dart_metadata = attributes.dart_metadata();

        Ok(IrStruct {
            name,
            wrapper_name,
            fields,
            is_fields_named,
            dart_metadata,
            ignore: attributes.ignore(),
            generate_hash: attributes.generate_hash(),
            generate_eq: attributes.generate_eq(),
            comments,
        })
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
            is_rust_public: Some(matches!(field.vis, Visibility::Public(_))),
            comments: parse_comments(&field.attrs),
            default: attributes.default_value(),
            settings: IrFieldSettings::default(),
        })
    }
}

struct EnumOrStructParserStruct<'a, 'b, 'c, 'd>(&'d mut TypeParserWithContext<'a, 'b, 'c>);

impl EnumOrStructParser<IrStructIdent, IrStruct, HirStruct, ItemStruct>
    for EnumOrStructParserStruct<'_, '_, '_, '_>
{
    fn parse_inner_impl(
        &mut self,
        src_object: &HirStruct,
        name: NamespacedName,
        wrapper_name: Option<String>,
    ) -> anyhow::Result<IrStruct> {
        self.0.parse_struct(src_object, name, wrapper_name)
    }

    fn construct_output(&self, ident: IrStructIdent) -> anyhow::Result<IrType> {
        Ok(StructRef(IrTypeStructRef {
            ident,
            is_exception: false,
        }))
    }

    fn src_objects(&self) -> &HashMap<String, &HirStruct> {
        &self.0.inner.src_structs
    }

    fn parser_info(&mut self) -> &mut EnumOrStructParserInfo<IrStructIdent, IrStruct> {
        &mut self.0.inner.struct_parser_info
    }

    fn dart_code_of_type(&mut self) -> &mut HashMap<String, String> {
        &mut self.0.inner.dart_code_of_type
    }

    fn parse_type_rust_auto_opaque_implicit(
        &mut self,
        namespace: Option<Namespace>,
        ty: &Type,
    ) -> anyhow::Result<IrType> {
        self.0.parse_type_rust_auto_opaque_implicit(namespace, ty)
    }

    fn compute_default_opaque(obj: &IrStruct) -> bool {
        structure_compute_default_opaque(obj)
    }
}

pub(super) fn structure_compute_default_opaque(s: &IrStruct) -> bool {
    (s.fields.iter()).any(|f| matches!(f.ty, IrType::RustAutoOpaqueImplicit(_)))
}
