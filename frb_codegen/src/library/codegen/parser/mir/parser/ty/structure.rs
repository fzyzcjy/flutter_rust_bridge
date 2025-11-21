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
use crate::codegen::parser::mir::parser::ty::generics::parse_generics_info;
use crate::codegen::parser::mir::parser::ty::misc::parse_comments;
use crate::codegen::parser::mir::parser::ty::type_substitution::{
    generate_concrete_name, has_concrete_generic_args, instantiate_struct, strip_path_generic_args,
};
use crate::codegen::parser::mir::parser::ty::unencodable::SplayedSegment;
use crate::codegen::parser::mir::parser::ty::{TypeParserParsingContext, TypeParserWithContext};
use crate::utils::basic_code::general_code::GeneralDartCode;
use crate::utils::crate_name::CrateName;
use crate::utils::namespace::{Namespace, NamespacedName};
use anyhow::bail;
use std::collections::HashMap;
use syn::{Field, Fields, FieldsNamed, FieldsUnnamed, ItemStruct, Type, Visibility};

impl TypeParserWithContext<'_, '_, '_> {
    pub(crate) fn parse_type_path_data_struct(
        &mut self,
        path: &syn::Path,
        last_segment: &SplayedSegment,
        override_opaque: Option<bool>,
    ) -> anyhow::Result<Option<MirType>> {
        self.parse_type_path_data_struct_with_alias(path, last_segment, override_opaque, None)
    }

    pub(crate) fn parse_type_path_data_struct_with_alias(
        &mut self,
        path: &syn::Path,
        last_segment: &SplayedSegment,
        override_opaque: Option<bool>,
        alias_name: Option<&str>,
    ) -> anyhow::Result<Option<MirType>> {
        let (name, generic_args) = last_segment;
        let namespace = self.parse_struct_namespace(name);

        if !generic_args.is_empty()
            && !has_concrete_generic_args(generic_args, &self.context.current_generic_params)
        {
            let template_path = strip_path_generic_args(path);
            let empty_args: &[syn::Type] = &[];
            return EnumOrStructParserStruct(self).parse(
                &template_path,
                &(name, empty_args),
                override_opaque,
            );
        }

        if !generic_args.is_empty() {
            if let Some(namespace) = namespace.clone() {
                let template_name = NamespacedName::new(namespace.clone(), name.to_string());
                let template_ident: MirStructIdent = template_name.clone().into();

                // Ensure template is parsed on demand
                if !self
                    .inner
                    .struct_parser_info
                    .generic_templates
                    .contains_key(&template_ident)
                {
                    let template_path = strip_path_generic_args(path);
                    let empty_args: &[syn::Type] = &[];
                    let _ = EnumOrStructParserStruct(self).parse(
                        &template_path,
                        &(name, empty_args),
                        override_opaque,
                    )?;
                }

                if let Some(template) = self
                    .inner
                    .struct_parser_info
                    .generic_templates
                    .get(&template_ident)
                    .cloned()
                {
                    if template.generic_params.len() != generic_args.len() {
                        let type_path = syn::TypePath { qself: None, path: path.clone() };
                        return Ok(Some(self.parse_type_rust_auto_opaque_implicit(
                            Some(namespace.clone()),
                            &syn::Type::Path(type_path),
                            None,
                            None,
                        )?));
                    }

                    let type_args: Vec<MirType> = generic_args
                        .iter()
                        .map(|arg_ty| self.parse_type(arg_ty))
                        .collect::<anyhow::Result<Vec<_>>>()?;

                    let concrete_name =
                        generate_concrete_name(namespace, name, alias_name, &type_args);
                    let concrete_ident: MirStructIdent = concrete_name.clone().into();

                    if self
                        .inner
                        .struct_parser_info
                        .object_pool
                        .contains_key(&concrete_ident)
                    {
                        return Ok(Some(MirType::StructRef(MirTypeStructRef {
                            ident: concrete_ident,
                            is_exception: false,
                        })));
                    }

                    let is_from_type_alias =
                        alias_name.is_some() || self.context.is_within_type_alias;
                    let concrete_struct = instantiate_struct(
                        &template,
                        concrete_name,
                        &type_args,
                        is_from_type_alias,
                    )?;

                    self.inner
                        .struct_parser_info
                        .object_pool
                        .insert(concrete_ident.clone(), concrete_struct);

                    return Ok(Some(MirType::StructRef(MirTypeStructRef {
                        ident: concrete_ident,
                        is_exception: false,
                    })));
                }
            }
        }

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

        // Extract generic parameters if this is a generic struct
        let generics_info = parse_generics_info(&src_struct.src.generics);
        let (generic_params, is_generic_template) = match generics_info {
            crate::codegen::parser::mir::parser::ty::generics::GenericsInfo::Generic { params } => {
                (params.clone(), true)
            }
            _ => (vec![], false),
        };

        let fields = struct_fields
            .iter()
            .enumerate()
            .map(|(idx, field)| self.parse_struct_field(idx, field, &attributes, &generic_params))
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
            ui_state: attributes.ui_state(),
            comments,
            generic_params,
            is_generic_template,
        })
    }

    fn parse_struct_field(
        &mut self,
        idx: usize,
        field: &Field,
        struct_attributes: &FrbAttributes,
        generic_params: &[String],
    ) -> anyhow::Result<MirField> {
        let field_name = field
            .ident
            .as_ref()
            .map_or(format!("field{idx}"), ToString::to_string);
        let field_type = self.parse_type_with_context(&field.ty, |c| {
            c.with_struct_or_enum_attributes(struct_attributes.clone())
                .with_generic_params(generic_params.to_vec())
        })?;
        let attributes = FrbAttributes::parse(&field.attrs)?;
        Ok(MirField {
            name: MirIdent::new(field_name, attributes.name()),
            ty: field_type,
            is_final: !attributes.non_final(),
            is_rust_public: Some(matches!(field.vis, Visibility::Public(_))),
            comments: parse_comments(&field.attrs),
            default: attributes.default_value(),
            settings: MirFieldSettings {
                skip_auto_accessors: (struct_attributes.ignore_all() || attributes.ignore())
                    && !attributes.unignore(),
                ..Default::default()
            },
        })
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

    fn is_generic_template(obj: &MirStruct) -> bool {
        obj.is_generic_template
    }
}

pub(super) fn structure_compute_default_opaque(s: &MirStruct, crate_name: &CrateName) -> bool {
    (s.fields.iter()).any(|f| {
        matches!(f.ty, MirType::RustAutoOpaqueImplicit(_))
            || ((!crate_name.is_self_crate()) && !f.is_rust_public.unwrap())
    })
}
