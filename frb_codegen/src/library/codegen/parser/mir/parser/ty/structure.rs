use crate::codegen::ir::hir::flat::struct_or_enum::HirFlatStruct;
use crate::codegen::ir::hir::misc::visibility::HirVisibility;
use crate::codegen::ir::mir::field::{MirField, MirFieldSettings};
use crate::codegen::ir::mir::ident::MirIdent;
use crate::codegen::ir::mir::ty::rust_auto_opaque_implicit::MirTypeRustAutoOpaqueImplicitReason;
use crate::codegen::ir::mir::ty::structure::{MirStruct, MirStructIdent, MirTypeStructRef};
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::ir::mir::ty::MirType::StructRef;
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
use crate::codegen::parser::mir::parser::ty::enum_or_struct::{
    parse_struct_or_enum_should_ignore, EnumOrStructParser, EnumOrStructParserInfo,
};
use crate::codegen::parser::mir::parser::ty::misc::parse_comments;
use crate::codegen::parser::mir::parser::ty::unencodable::SplayedSegment;
use crate::codegen::parser::mir::parser::ty::{TypeParserParsingContext, TypeParserWithContext};
use crate::utils::basic_code::general_code::GeneralDartCode;
use crate::utils::crate_name::CrateName;
use crate::utils::namespace::{Namespace, NamespacedName};
use anyhow::bail;
use std::collections::HashMap;
use syn::visit_mut::VisitMut;
use syn::{Field, Fields, FieldsNamed, FieldsUnnamed, ItemStruct, Type, TypePath, Visibility};

impl TypeParserWithContext<'_, '_, '_> {
    pub(crate) fn parse_type_path_data_struct(
        &mut self,
        path: &syn::Path,
        last_segment: &SplayedSegment,
        override_opaque: Option<bool>,
    ) -> anyhow::Result<Option<MirType>> {
        EnumOrStructParserStruct(self).parse(path, last_segment, override_opaque)
    }

    pub(crate) fn parse_struct_namespace(&mut self, name: &str) -> Option<Namespace> {
        EnumOrStructParserStruct(self).parse_namespace(name)
    }

    fn parse_struct(
        &mut self,
        src_struct: &HirFlatStruct,
        name: NamespacedName,
        wrapper_name: Option<String>,
    ) -> anyhow::Result<MirStruct> {
        let (is_fields_named, struct_fields) = match &src_struct.src.fields {
            Fields::Named(FieldsNamed { named, .. }) => (true, named),
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => (false, unnamed),
            // This will stop the whole generator and tell the users, so we do not care about testing it
            // frb-coverage:ignore-start
            Fields::Unit => bail!("struct with unit fields are not supported yet, what about using `struct {name} {{}}` or `#[frb(opaque)] struct {name};` instead", name = src_struct.name.name),
            // frb-coverage:ignore-end
        };

        let attributes = FrbAttributes::parse(&src_struct.src.attrs)?;
        let dart_metadata = attributes.dart_metadata();

        let fields = struct_fields
            .iter()
            .enumerate()
            .map(|(idx, field)| self.parse_struct_field(idx, field, &attributes))
            .collect::<anyhow::Result<Vec<_>>>()?;

        let comments = parse_comments(&src_struct.src.attrs);

        let ignore = parse_struct_or_enum_should_ignore(
            src_struct,
            &name.namespace.crate_name(),
            self.context,
        );

        Ok(MirStruct {
            name,
            wrapper_name,
            fields,
            is_fields_named,
            dart_metadata_raw: dart_metadata,
            ignore,
            needs_json_serializable: attributes.json_serializable(),
            generate_hash: attributes.generate_hash(),
            generate_eq: attributes.generate_eq(),
            dart_collection_deep_equality: attributes.dart_collection_deep_equality(),
            ui_state: attributes.ui_state(),
            comments,
        })
    }

    fn parse_struct_field(
        &mut self,
        idx: usize,
        field: &Field,
        struct_attributes: &FrbAttributes,
    ) -> anyhow::Result<MirField> {
        let field_name = field
            .ident
            .as_ref()
            .map_or(format!("field{idx}"), ToString::to_string);
        let field_type = self.parse_type_with_context(&field.ty, |c| {
            c.with_struct_or_enum_attributes(struct_attributes.clone())
        })?;
        let attributes = FrbAttributes::parse(&field.attrs)?;
        let contains_inaccessible_private_type = self.contains_inaccessible_private_type(&field.ty);
        Ok(MirField {
            name: MirIdent::new(field_name, attributes.name()),
            ty: field_type,
            is_final: !attributes.non_final(),
            is_rust_public: Some(matches!(field.vis, Visibility::Public(_))),
            comments: parse_comments(&field.attrs),
            default: attributes.default_value(),
            settings: MirFieldSettings {
                skip_auto_accessors: ((struct_attributes.ignore_all() || attributes.ignore())
                    && !attributes.unignore())
                    || contains_inaccessible_private_type,
                ..Default::default()
            },
        })
    }

    fn contains_inaccessible_private_type(&self, ty: &Type) -> bool {
        let mut ty = ty.clone();
        let mut visitor = InaccessiblePrivateTypeVisitor {
            src_structs: &self.inner.src_structs,
            src_enums: &self.inner.src_enums,
            rust_output_path_namespace: &self.context.rust_output_path_namespace,
            found: false,
        };
        visitor.visit_type_mut(&mut ty);
        visitor.found
    }
}

struct InaccessiblePrivateTypeVisitor<'a, 'b> {
    src_structs: &'a HashMap<String, &'b HirFlatStruct>,
    src_enums: &'a HashMap<String, &'b crate::codegen::ir::hir::flat::struct_or_enum::HirFlatEnum>,
    rust_output_path_namespace: &'a Namespace,
    found: bool,
}

impl VisitMut for InaccessiblePrivateTypeVisitor<'_, '_> {
    fn visit_type_path_mut(&mut self, node: &mut TypePath) {
        if self.found {
            return;
        }

        if let Some(name) = node
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string())
        {
            self.found =
                self.src_structs.get(&name).is_some_and(|item| {
                    self.is_inaccessible(item.visibility, &item.name.namespace)
                }) || self.src_enums.get(&name).is_some_and(|item| {
                    self.is_inaccessible(item.visibility, &item.name.namespace)
                });
        }

        syn::visit_mut::visit_type_path_mut(self, node);
    }
}

impl InaccessiblePrivateTypeVisitor<'_, '_> {
    fn is_inaccessible(&self, visibility: HirVisibility, namespace: &Namespace) -> bool {
        visibility == HirVisibility::Inherited
            && !namespace.is_prefix_of(self.rust_output_path_namespace)
    }
}

struct EnumOrStructParserStruct<'a, 'b, 'c, 'd>(&'d mut TypeParserWithContext<'a, 'b, 'c>);

impl EnumOrStructParser<MirStructIdent, MirStruct, ItemStruct>
    for EnumOrStructParserStruct<'_, '_, '_, '_>
{
    fn parse_inner_impl(
        &mut self,
        src_object: &HirFlatStruct,
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

    fn src_objects(&self) -> &HashMap<String, &HirFlatStruct> {
        &self.0.inner.src_structs
    }

    fn parser_info(&mut self) -> &mut EnumOrStructParserInfo<MirStructIdent, MirStruct> {
        &mut self.0.inner.struct_parser_info
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

    fn compute_default_opaque(obj: &MirStruct) -> bool {
        structure_compute_default_opaque(obj, &obj.name.namespace.crate_name())
    }
}

pub(super) fn structure_compute_default_opaque(s: &MirStruct, crate_name: &CrateName) -> bool {
    (s.fields.iter()).any(|f| {
        matches!(f.ty, MirType::RustAutoOpaqueImplicit(_))
            || ((!crate_name.is_self_crate()) && !f.is_rust_public.unwrap())
    })
}
