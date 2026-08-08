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
use syn::{
    Field, Fields, FieldsNamed, FieldsUnnamed, ItemStruct, ItemUse, Type, TypePath, UseTree,
    Visibility,
};

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
            .map(|(idx, field)| {
                self.parse_struct_field(
                    idx,
                    field,
                    &attributes,
                    &src_struct.declaration_namespace,
                    &src_struct.imports,
                )
            })
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
        struct_namespace: &Namespace,
        imports: &[ItemUse],
    ) -> anyhow::Result<MirField> {
        let field_name = field
            .ident
            .as_ref()
            .map_or(format!("field{idx}"), ToString::to_string);
        let field_type = self.parse_type_with_context(&field.ty, |c| {
            c.with_struct_or_enum_attributes(struct_attributes.clone())
        })?;
        let attributes = FrbAttributes::parse(&field.attrs)?;
        let contains_inaccessible_private_type =
            self.contains_inaccessible_private_type(&field.ty, struct_namespace, imports);
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

    fn contains_inaccessible_private_type(
        &self,
        ty: &Type,
        struct_namespace: &Namespace,
        imports: &[ItemUse],
    ) -> bool {
        let mut ty = ty.clone();
        let mut visitor = InaccessiblePrivateTypeVisitor {
            src_structs: &self.inner.src_structs,
            src_enums: &self.inner.src_enums,
            src_types: &self.inner.src_types,
            src_generic_type_aliases: &self.inner.src_generic_type_aliases,
            initiated_namespace: struct_namespace,
            imports,
            alias_depth: 0,
            found: false,
        };
        visitor.visit_type_mut(&mut ty);
        visitor.found
    }
}

struct InaccessiblePrivateTypeVisitor<'a, 'b> {
    src_structs: &'a HashMap<String, &'b HirFlatStruct>,
    src_enums: &'a HashMap<String, &'b crate::codegen::ir::hir::flat::struct_or_enum::HirFlatEnum>,
    src_types: &'a HashMap<
        String,
        crate::codegen::ir::hir::flat::type_alias::HirFlatTypeAlias,
    >,
    src_generic_type_aliases:
        &'a HashMap<String, crate::codegen::ir::hir::flat::type_alias::HirFlatTypeAlias>,
    initiated_namespace: &'a Namespace,
    imports: &'a [ItemUse],
    alias_depth: usize,
    found: bool,
}

impl VisitMut for InaccessiblePrivateTypeVisitor<'_, '_> {
    fn visit_type_path_mut(&mut self, node: &mut TypePath) {
        if self.found {
            return;
        }

        let candidates = type_path_candidates(&node.path, self.initiated_namespace, self.imports);
        self.found = candidates.iter().any(|candidate| {
            self.src_structs
                .get(&candidate.name)
                .is_some_and(|item| !item.is_accessible_from_rust_output && item.name == *candidate)
                || self.src_enums.get(&candidate.name).is_some_and(|item| {
                    !item.is_accessible_from_rust_output && item.name == *candidate
                })
        });

        if !self.found && self.alias_depth < 64 {
            for candidate in candidates {
                let alias = self
                    .src_types
                    .get(&candidate.name)
                    .or_else(|| {
                        self.src_generic_type_aliases
                            .get(&candidate.name)
                    })
                    .filter(|alias| alias.namespace == candidate.namespace);
                if let Some(alias) = alias {
                    let mut alias_target = alias.target.clone();
                    let mut alias_visitor = InaccessiblePrivateTypeVisitor {
                        src_structs: self.src_structs,
                        src_enums: self.src_enums,
                        src_types: self.src_types,
                        src_generic_type_aliases: self.src_generic_type_aliases,
                        initiated_namespace: &alias.declaration_namespace,
                        imports: &alias.imports,
                        alias_depth: self.alias_depth + 1,
                        found: false,
                    };
                    alias_visitor.visit_type_mut(&mut alias_target);
                    if alias_visitor.found {
                        self.found = true;
                        break;
                    }
                }
            }
        }

        syn::visit_mut::visit_type_path_mut(self, node);
    }
}

#[cfg(test)]
fn type_path_targets_item(
    path: &syn::Path,
    item_name: &NamespacedName,
    initiated_namespace: &Namespace,
    imports: &[ItemUse],
) -> bool {
    type_path_candidates(path, initiated_namespace, imports).contains(item_name)
}

fn type_path_candidates(
    path: &syn::Path,
    initiated_namespace: &Namespace,
    imports: &[ItemUse],
) -> Vec<NamespacedName> {
    let segments = path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>();
    if segments.len() == 1 {
        let mut output = vec![NamespacedName::new(
            initiated_namespace.clone(),
            segments[0].clone(),
        )];
        for item_use in imports {
            collect_import_targets(
                &item_use.tree,
                &[],
                &segments[0],
                initiated_namespace,
                &mut output,
            );
        }
        output.sort();
        output.dedup();
        return output;
    }

    let type_namespace_segments = &segments[..segments.len() - 1];
    let mut output = vec![];
    if let Some(namespace) =
        resolve_relative_namespace(type_namespace_segments, initiated_namespace)
    {
        output.push(NamespacedName::new(
            namespace,
            segments.last().unwrap().clone(),
        ));
    }
    output.push(NamespacedName::new(
        Namespace::new(type_namespace_segments.to_vec()),
        segments.last().unwrap().clone(),
    ));
    let mut imported_modules = vec![];
    for item_use in imports {
        collect_import_targets(
            &item_use.tree,
            &[],
            &type_namespace_segments[0],
            initiated_namespace,
            &mut imported_modules,
        );
    }
    for imported_module in imported_modules {
        let mut namespace = imported_module.namespace.join(&imported_module.name);
        for segment in &type_namespace_segments[1..] {
            namespace = namespace.join(segment);
        }
        output.push(NamespacedName::new(
            namespace,
            segments.last().unwrap().clone(),
        ));
    }
    output.sort();
    output.dedup();
    output
}

fn collect_import_targets(
    tree: &UseTree,
    prefix: &[String],
    local_name: &str,
    initiated_namespace: &Namespace,
    output: &mut Vec<NamespacedName>,
) {
    match tree {
        UseTree::Path(inner) => {
            let mut child_prefix = prefix.to_vec();
            child_prefix.push(inner.ident.to_string());
            collect_import_targets(
                &inner.tree,
                &child_prefix,
                local_name,
                initiated_namespace,
                output,
            )
        }
        UseTree::Name(inner) => {
            if inner.ident == local_name {
                push_import_targets(prefix, inner.ident.to_string(), initiated_namespace, output);
            }
        }
        UseTree::Rename(inner) => {
            if inner.rename == local_name {
                push_import_targets(prefix, inner.ident.to_string(), initiated_namespace, output);
            }
        }
        UseTree::Glob(_) => {
            push_import_targets(prefix, local_name.to_owned(), initiated_namespace, output);
        }
        UseTree::Group(inner) => {
            for tree in &inner.items {
                collect_import_targets(tree, prefix, local_name, initiated_namespace, output);
            }
        }
    }
}

fn push_import_targets(
    prefix: &[String],
    item_name: String,
    initiated_namespace: &Namespace,
    output: &mut Vec<NamespacedName>,
) {
    if let Some(namespace) = resolve_relative_namespace(prefix, initiated_namespace) {
        output.push(NamespacedName::new(namespace, item_name.clone()));
    }
    output.push(NamespacedName::new(
        Namespace::new(prefix.to_vec()),
        item_name,
    ));
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
            &[],
        ));
        assert!(!type_path_targets_item(
            &syn::parse_str::<TypePath>("String").unwrap().path,
            &other_name,
            &initiated_namespace,
            &[],
        ));
        assert!(type_path_targets_item(
            &syn::parse_str::<TypePath>("crate::api::other::String")
                .unwrap()
                .path,
            &other_name,
            &initiated_namespace,
            &[],
        ));
        assert!(!type_path_targets_item(
            &syn::parse_str::<TypePath>("std::string::String")
                .unwrap()
                .path,
            &other_name,
            &initiated_namespace,
            &[],
        ));

        let private_import = syn::parse_str::<ItemUse>("use super::other::String;").unwrap();
        assert!(type_path_targets_item(
            &syn::parse_str::<TypePath>("String").unwrap().path,
            &other_name,
            &initiated_namespace,
            &[private_import],
        ));

        let renamed_import =
            syn::parse_str::<ItemUse>("use super::other::String as Renamed;").unwrap();
        assert!(type_path_targets_item(
            &syn::parse_str::<TypePath>("Renamed").unwrap().path,
            &other_name,
            &initiated_namespace,
            &[renamed_import],
        ));

        let struct_namespace = Namespace::new_self_crate("api::models".to_owned());
        let nested_name = NamespacedName::new(
            Namespace::new_self_crate("api::models::hidden".to_owned()),
            "Inner".to_owned(),
        );
        assert!(type_path_targets_item(
            &syn::parse_str::<TypePath>("hidden::Inner").unwrap().path,
            &nested_name,
            &struct_namespace,
            &[],
        ));

        let renamed_module_import =
            syn::parse_str::<ItemUse>("use super::other as renamed;").unwrap();
        assert!(type_path_targets_item(
            &syn::parse_str::<TypePath>("renamed::String")
                .unwrap()
                .path,
            &other_name,
            &initiated_namespace,
            &[renamed_module_import],
        ));
    }

    /// Resolves an imported alias target with imports from the alias declaration module.
    #[test]
    fn test_imported_alias_target_uses_declaration_imports() {
        use crate::codegen::ir::hir::flat::type_alias::HirFlatTypeAlias;
        use crate::codegen::ir::hir::misc::generation_source::HirGenerationSource;
        use crate::codegen::ir::hir::misc::visibility::HirVisibility;

        let hidden_namespace = Namespace::new_self_crate("api::hidden".to_owned());
        let inner = HirFlatStruct {
            name: NamespacedName::new(hidden_namespace.clone(), "Inner".to_owned()),
            declaration_namespace: hidden_namespace,
            visibility: HirVisibility::Public,
            is_accessible_from_rust_output: false,
            imports: vec![],
            sources: vec![HirGenerationSource::Normal],
            mirror: false,
            src: syn::parse_str("pub struct Inner { pub value: String }").unwrap(),
        };
        let aliases_namespace = Namespace::new_self_crate("api::aliases".to_owned());
        let alias = HirFlatTypeAlias {
            ident: "Alias".to_owned(),
            namespace: aliases_namespace.clone(),
            declaration_namespace: aliases_namespace,
            imports: vec![syn::parse_str("use super::hidden::Inner;").unwrap()],
            target: syn::parse_str("Inner").unwrap(),
            type_params: vec![],
        };
        let src_structs = HashMap::from([("Inner".to_owned(), &inner)]);
        let src_enums = HashMap::new();
        let src_types = HashMap::from([("Alias".to_owned(), alias)]);
        let src_generic_type_aliases = HashMap::new();
        let initiated_namespace = Namespace::new_self_crate("api".to_owned());
        let imports = vec![syn::parse_str("use aliases::Alias;").unwrap()];
        let mut ty = syn::parse_str("Arc<Alias>").unwrap();
        let mut visitor = InaccessiblePrivateTypeVisitor {
            src_structs: &src_structs,
            src_enums: &src_enums,
            src_types: &src_types,
            src_generic_type_aliases: &src_generic_type_aliases,
            initiated_namespace: &initiated_namespace,
            imports: &imports,
            alias_depth: 0,
            found: false,
        };

        visitor.visit_type_mut(&mut ty);

        assert!(visitor.found);
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
