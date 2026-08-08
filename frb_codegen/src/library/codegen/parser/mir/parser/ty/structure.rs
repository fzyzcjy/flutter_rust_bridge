use crate::codegen::ir::hir::flat::struct_or_enum::HirFlatStruct;
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
        let resolved_field_type = self.resolve_alias(&field.ty);
        let contains_inaccessible_private_type =
            self.contains_inaccessible_private_type(&resolved_field_type);
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
            initiated_namespace: &self.context.initiated_namespace,
            found: false,
        };
        visitor.visit_type_mut(&mut ty);
        visitor.found
    }
}

struct InaccessiblePrivateTypeVisitor<'a, 'b> {
    src_structs: &'a HashMap<String, &'b HirFlatStruct>,
    src_enums: &'a HashMap<String, &'b crate::codegen::ir::hir::flat::struct_or_enum::HirFlatEnum>,
    initiated_namespace: &'a Namespace,
    found: bool,
}

impl VisitMut for InaccessiblePrivateTypeVisitor<'_, '_> {
    fn visit_type_path_mut(&mut self, node: &mut TypePath) {
        if self.found {
            return;
        }

        if let Some(name) = node.path.segments.last().map(|x| x.ident.to_string()) {
            self.found = self.src_structs.get(&name).is_some_and(|item| {
                !item.is_accessible_from_rust_output
                    && type_path_targets_item(&node.path, &item.name, self.initiated_namespace)
            }) || self.src_enums.get(&name).is_some_and(|item| {
                !item.is_accessible_from_rust_output
                    && type_path_targets_item(&node.path, &item.name, self.initiated_namespace)
            });
        }

        syn::visit_mut::visit_type_path_mut(self, node);
    }
}

fn type_path_targets_item(
    path: &syn::Path,
    item_name: &NamespacedName,
    initiated_namespace: &Namespace,
) -> bool {
    let segments = path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>();
    if segments.len() == 1 {
        return item_name.namespace == *initiated_namespace;
    }

    let type_namespace_segments = &segments[..segments.len() - 1];
    resolve_relative_namespace(type_namespace_segments, initiated_namespace)
        .is_some_and(|namespace| namespace == item_name.namespace)
        || Namespace::new(type_namespace_segments.to_vec()) == item_name.namespace
}

fn resolve_relative_namespace(
    type_namespace_segments: &[String],
    initiated_namespace: &Namespace,
) -> Option<Namespace> {
    let mut output = initiated_namespace
        .path()
        .into_iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    let mut index = 0;

    match type_namespace_segments.first().map(String::as_str) {
        Some("crate") => {
            output = vec!["crate".to_owned()];
            index = 1;
        }
        Some("self") => index = 1,
        Some("super") => {
            while type_namespace_segments.get(index).map(String::as_str) == Some("super") {
                output.pop()?;
                index += 1;
            }
        }
        _ => {}
    }

    output.extend(type_namespace_segments[index..].iter().cloned());
    Some(Namespace::new(output))
}

#[cfg(test)]
mod inaccessible_private_type_tests {
    use super::*;

    /// Resolves local and qualified type paths without matching unrelated names.
    #[test]
    fn test_type_path_targets_item_by_namespace() {
        let initiated_namespace = Namespace::new_self_crate("api::current".to_owned());
        let local_name = NamespacedName::new(initiated_namespace.clone(), "String".to_owned());
        let other_name = NamespacedName::new(
            Namespace::new_self_crate("api::other".to_owned()),
            "String".to_owned(),
        );

        assert!(type_path_targets_item(
            &syn::parse_str::<TypePath>("String").unwrap().path,
            &local_name,
            &initiated_namespace,
        ));
        assert!(!type_path_targets_item(
            &syn::parse_str::<TypePath>("String").unwrap().path,
            &other_name,
            &initiated_namespace,
        ));
        assert!(type_path_targets_item(
            &syn::parse_str::<TypePath>("crate::api::other::String")
                .unwrap()
                .path,
            &other_name,
            &initiated_namespace,
        ));
        assert!(!type_path_targets_item(
            &syn::parse_str::<TypePath>("std::string::String")
                .unwrap()
                .path,
            &other_name,
            &initiated_namespace,
        ));
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
