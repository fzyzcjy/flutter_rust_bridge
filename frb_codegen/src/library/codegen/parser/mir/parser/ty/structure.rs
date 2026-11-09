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
use std::collections::{HashMap, HashSet};
use syn::{Field, Fields, FieldsNamed, FieldsUnnamed, ItemStruct, Type, Visibility};

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

    fn dart_code_mixin_opt_in(&mut self) -> &mut HashSet<String> {
        &mut self.0.inner.dart_code_mixin_opt_in
    }

    fn dart_code_mixin_blocked(&mut self) -> &mut HashSet<String> {
        &mut self.0.inner.dart_code_mixin_blocked
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

#[cfg(test)]
mod tests {
    use super::super::enum_or_struct::EnumOrStructParser;
    use super::super::{empty_type_parser, test_parsing_context, TypeParserWithContext};
    use super::EnumOrStructParserStruct;
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use crate::codegen::ir::mir::ty::MirType;
    use crate::codegen::ir::mir::ty::MirTypeTrait;
    use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
    use syn::parse_quote;

    /// Records both opt-in and the ui_state block when a struct carries `dart_code`.
    #[test]
    fn records_mixin_and_ui_state_flags_from_dart_code() {
        let mut parser = empty_type_parser();
        let context = test_parsing_context();
        let item: syn::ItemStruct = parse_quote! {
            #[frb(dart_code = "int extra() => 1;", dart_code_mixin, ui_state)]
            struct Value;
        };
        let raw = Some((
            MirType::Primitive(MirTypePrimitive::I32),
            FrbAttributes::parse(&item.attrs).unwrap(),
        ));
        {
            let mut with_context = TypeParserWithContext::new(&mut parser, &context);
            EnumOrStructParserStruct(&mut with_context).record_dart_code_for_test(&raw);
        }
        let key = MirType::Primitive(MirTypePrimitive::I32).safe_ident();
        assert!(parser.dart_code_mixin_opt_in.contains(&key));
        assert!(parser.dart_code_mixin_blocked.contains(&key));
        assert!(parser.dart_code_of_type.contains_key(&key));
    }
}
