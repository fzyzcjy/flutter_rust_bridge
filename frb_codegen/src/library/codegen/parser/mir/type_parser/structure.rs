use crate::codegen::ir::hir::hierarchical::struct_or_enum::HirStruct;
use crate::codegen::ir::mir::field::{MirField, MirFieldSettings};
use crate::codegen::ir::mir::ident::MirIdent;
use crate::codegen::ir::mir::ty::structure::{MirStruct, MirStructIdent, MirTypeStructRef};
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::ir::mir::ty::MirType::StructRef;
use crate::codegen::parser::mir::attribute_parser::FrbAttributes;
use crate::codegen::parser::mir::type_parser::enum_or_struct::{
    EnumOrStructParser, EnumOrStructParserInfo,
};
use crate::codegen::parser::mir::type_parser::misc::{parse_comments, parse_simple_type_should_ignore};
use crate::codegen::parser::mir::type_parser::unencodable::SplayedSegment;
use crate::codegen::parser::mir::type_parser::TypeParserWithContext;
use crate::utils::namespace::{Namespace, NamespacedName};
use anyhow::bail;
use std::collections::HashMap;
use syn::{Field, Fields, FieldsNamed, FieldsUnnamed, ItemStruct, Type, Visibility};

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path_data_struct(
        &mut self,
        last_segment: &SplayedSegment,
        override_opaque: Option<bool>,
    ) -> anyhow::Result<Option<MirType>> {
        EnumOrStructParserStruct(self).parse(last_segment, override_opaque)
    }

    fn parse_struct(
        &mut self,
        src_struct: &HirStruct,
        name: NamespacedName,
        wrapper_name: Option<String>,
    ) -> anyhow::Result<MirStruct> {
        let (is_fields_named, struct_fields) = match &src_struct.src.fields {
            Fields::Named(FieldsNamed { named, .. }) => (true, named),
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => (false, unnamed),
            // This will stop the whole generator and tell the users, so we do not care about testing it
            // frb-coverage:ignore-start
            Fields::Unit => bail!("struct with unit fields are not supported yet, what about using `struct {} {{}}` instead", src_struct.ident),
            // frb-coverage:ignore-end
        };

        let fields = struct_fields
            .iter()
            .enumerate()
            .map(|(idx, field)| self.parse_struct_field(idx, field))
            .collect::<anyhow::Result<Vec<_>>>()?;

        let comments = parse_comments(&src_struct.src.attrs);

        let attributes = FrbAttributes::parse(&src_struct.src.attrs)?;
        let dart_metadata = attributes.dart_metadata();

        let ignore = parse_simple_type_should_ignore(&attributes, src_struct.visibility);

        Ok(MirStruct {
            name,
            wrapper_name,
            fields,
            is_fields_named,
            dart_metadata,
            ignore,
            generate_hash: attributes.generate_hash(),
            generate_eq: attributes.generate_eq(),
            comments,
        })
    }

    fn parse_struct_field(&mut self, idx: usize, field: &Field) -> anyhow::Result<MirField> {
        let field_name = field
            .ident
            .as_ref()
            .map_or(format!("field{idx}"), ToString::to_string);
        let field_type = self.parse_type(&field.ty)?;
        let attributes = FrbAttributes::parse(&field.attrs)?;
        Ok(MirField {
            name: MirIdent::new(field_name),
            ty: field_type,
            is_final: !attributes.non_final(),
            is_rust_public: Some(matches!(field.vis, Visibility::Public(_))),
            comments: parse_comments(&field.attrs),
            default: attributes.default_value(),
            settings: MirFieldSettings::default(),
        })
    }
}

struct EnumOrStructParserStruct<'a, 'b, 'c, 'd>(&'d mut TypeParserWithContext<'a, 'b, 'c>);

impl EnumOrStructParser<MirStructIdent, MirStruct, ItemStruct>
    for EnumOrStructParserStruct<'_, '_, '_, '_>
{
    fn parse_inner_impl(
        &mut self,
        src_object: &HirStruct,
        name: NamespacedName,
        wrapper_name: Option<String>,
    ) -> anyhow::Result<MirStruct> {
        self.0.parse_struct(src_object, name, wrapper_name)
    }

    fn construct_output(&self, ident: MirStructIdent) -> anyhow::Result<MirType> {
        Ok(StructRef(MirTypeStructRef {
            ident,
            is_exception: false,
        }))
    }

    fn src_objects(&self) -> &HashMap<String, &HirStruct> {
        &self.0.inner.src_structs
    }

    fn parser_info(&mut self) -> &mut EnumOrStructParserInfo<MirStructIdent, MirStruct> {
        &mut self.0.inner.struct_parser_info
    }

    fn dart_code_of_type(&mut self) -> &mut HashMap<String, String> {
        &mut self.0.inner.dart_code_of_type
    }

    fn parse_type_rust_auto_opaque_implicit(
        &mut self,
        namespace: Option<Namespace>,
        ty: &Type,
        override_ignore: Option<bool>,
    ) -> anyhow::Result<MirType> {
        self.0.parse_type_rust_auto_opaque_implicit(namespace, ty, override_ignore)
    }

    fn compute_default_opaque(obj: &MirStruct) -> bool {
        structure_compute_default_opaque(obj)
    }
}

pub(super) fn structure_compute_default_opaque(s: &MirStruct) -> bool {
    (s.fields.iter()).any(|f| matches!(f.ty, MirType::RustAutoOpaqueImplicit(_)))
}
