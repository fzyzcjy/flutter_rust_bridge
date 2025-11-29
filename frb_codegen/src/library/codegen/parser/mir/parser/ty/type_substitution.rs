use crate::codegen::ir::mir::field::MirField;
use crate::codegen::ir::mir::ty::enumeration::{MirEnum, MirEnumVariant, MirVariantKind};
use crate::codegen::ir::mir::ty::structure::MirStruct;
use crate::codegen::ir::mir::ty::MirType;
use crate::utils::namespace::NamespacedName;
use std::collections::HashMap;

/// Substitutes generic type parameters with concrete types in a MirEnum template
pub(crate) fn instantiate_enum(
    template: &MirEnum,
    concrete_name: NamespacedName,
    type_args: &[MirType],
    is_from_type_alias: bool,
) -> anyhow::Result<MirEnum> {
    if template.generic_params.len() != type_args.len() {
        anyhow::bail!(
            "Mismatch in generic parameter count: template has {}, but {} type arguments provided",
            template.generic_params.len(),
            type_args.len()
        );
    }

    // Create mapping from generic parameter names to concrete types
    let substitution_map: HashMap<String, MirType> = template
        .generic_params
        .iter()
        .zip(type_args.iter())
        .map(|(param_name, concrete_type)| (param_name.clone(), concrete_type.clone()))
        .collect();

    // Substitute types in variants
    let substituted_variants = template
        .variants
        .iter()
        .map(|variant| {
            let substituted_kind = match &variant.kind {
                MirVariantKind::Value => MirVariantKind::Value,
                MirVariantKind::Struct(struct_data) => {
                    let substituted_fields = struct_data
                        .fields
                        .iter()
                        .map(|field| {
                            Ok(MirField {
                                ty: substitute_type(&field.ty, &substitution_map)?,
                                name: field.name.clone(),
                                is_final: field.is_final,
                                is_rust_public: field.is_rust_public,
                                comments: field.comments.clone(),
                                default: field.default.clone(),
                                settings: field.settings.clone(),
                            })
                        })
                        .collect::<anyhow::Result<Vec<_>>>()?;

                    let substituted_struct = MirStruct {
                        name: struct_data.name.clone(),
                        wrapper_name: struct_data.wrapper_name.clone(),
                        fields: substituted_fields,
                        is_fields_named: struct_data.is_fields_named,
                        dart_metadata_raw: struct_data.dart_metadata_raw.clone(),
                        ignore: struct_data.ignore,
                        needs_json_serializable: struct_data.needs_json_serializable,
                        generate_hash: struct_data.generate_hash,
                        generate_eq: struct_data.generate_eq,
                        ui_state: struct_data.ui_state,
                        comments: struct_data.comments.clone(),
                        generic_params: vec![],
                        is_generic_template: false,
                    };
                    MirVariantKind::Struct(substituted_struct)
                }
            };
            // Update wrapper name to use concrete enum name instead of template name
            let wrapper_name_str = variant.wrapper_name.rust_style(true);
            let updated_wrapper_name = if wrapper_name_str.starts_with(&template.name.name) {
                // Replace template enum name with concrete enum name in wrapper name
                let variant_part = &wrapper_name_str[template.name.name.len()..];
                format!("{}{}", concrete_name.name, variant_part)
            } else {
                wrapper_name_str
            };

            Ok(MirEnumVariant {
                name: variant.name.clone(),
                wrapper_name: crate::codegen::ir::mir::ident::MirIdent::new(updated_wrapper_name, None),
                comments: variant.comments.clone(),
                kind: substituted_kind,
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    // Update wrapper_name to use concrete enum name instead of template name
    let updated_wrapper_name = if let Some(ref wrapper) = template.wrapper_name {
        let template_name_rust = template.name.rust_style();
        let concrete_name_rust = concrete_name.rust_style();
        // Replace template name with concrete name in wrapper string
        if wrapper.contains(&template_name_rust) {
            Some(wrapper.replace(&template_name_rust, &concrete_name_rust))
        } else {
            template.wrapper_name.clone()
        }
    } else {
        template.wrapper_name.clone()
    };

    // If this enum is instantiated through a type alias, it should not be ignored
    // even if the template enum is ignored. This allows ignored generic enums to
    // generate Dart code when used through type aliases.
    let ignore = if is_from_type_alias {
        false
    } else {
        template.ignore
    };

    Ok(MirEnum {
        name: concrete_name,
        wrapper_name: updated_wrapper_name,
        comments: template.comments.clone(),
        variants: substituted_variants,
        mode: template.mode,
        ignore,
        needs_json_serializable: template.needs_json_serializable,
        generic_params: vec![], // Now concrete, no generic params
        is_generic_template: false, // This is a concrete instance
    })
}

/// Substitutes generic type parameters with concrete types in a MirStruct template
pub(crate) fn instantiate_struct(
    template: &MirStruct,
    concrete_name: NamespacedName,
    type_args: &[MirType],
    is_from_type_alias: bool,
) -> anyhow::Result<MirStruct> {
    if template.generic_params.len() != type_args.len() {
        anyhow::bail!(
            "Mismatch in generic parameter count: struct template has {}, but {} type arguments provided",
            template.generic_params.len(),
            type_args.len()
        );
    }

    // Create mapping from generic parameter names to concrete types
    let substitution_map: HashMap<String, MirType> = template
        .generic_params
        .iter()
        .zip(type_args.iter())
        .map(|(param_name, concrete_type)| (param_name.clone(), concrete_type.clone()))
        .collect();

    // Substitute types in fields
    let substituted_fields = template
        .fields
        .iter()
        .map(|field| {
            Ok(MirField {
                ty: substitute_type(&field.ty, &substitution_map)?,
                name: field.name.clone(),
                is_final: field.is_final,
                is_rust_public: field.is_rust_public,
                comments: field.comments.clone(),
                default: field.default.clone(),
                settings: field.settings.clone(),
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    // Update wrapper_name to use concrete struct name instead of template name
    let updated_wrapper_name = if let Some(ref wrapper) = template.wrapper_name {
        let template_name_rust = template.name.rust_style();
        let concrete_name_rust = concrete_name.rust_style();
        // Replace template name with concrete name in wrapper string
        if wrapper.contains(&template_name_rust) {
            Some(wrapper.replace(&template_name_rust, &concrete_name_rust))
        } else {
            template.wrapper_name.clone()
        }
    } else {
        template.wrapper_name.clone()
    };

    // If this struct is instantiated through a type alias, it should not be ignored
    // even if the template struct is ignored. This allows ignored generic structs to
    // generate Dart code when used through type aliases.
    let ignore = if is_from_type_alias {
        false
    } else {
        template.ignore
    };

    Ok(MirStruct {
        name: concrete_name,
        wrapper_name: updated_wrapper_name,
        fields: substituted_fields,
        is_fields_named: template.is_fields_named,
        dart_metadata_raw: template.dart_metadata_raw.clone(),
        ignore,
        needs_json_serializable: template.needs_json_serializable,
        generate_hash: template.generate_hash,
        generate_eq: template.generate_eq,
        ui_state: template.ui_state,
        comments: template.comments.clone(),
        generic_params: vec![], // Now concrete, no generic params
        is_generic_template: false, // This is a concrete instance
    })
}

/// Recursively substitute generic type parameters with concrete types
fn substitute_type(
    ty: &MirType,
    substitution_map: &HashMap<String, MirType>,
) -> anyhow::Result<MirType> {
    match ty {
        MirType::Generic(generic) => {
            // Replace generic placeholder with concrete type
            substitution_map
                .get(&generic.param_name)
                .cloned()
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "Generic parameter '{}' not found in substitution map",
                        generic.param_name
                    )
                })
        }
        MirType::Boxed(boxed) => Ok(MirType::Boxed(crate::codegen::ir::mir::ty::boxed::MirTypeBoxed {
            exist_in_real_api: boxed.exist_in_real_api,
            inner: Box::new(substitute_type(&boxed.inner, substitution_map)?),
        })),
        MirType::Optional(opt) => Ok(MirType::Optional(
            crate::codegen::ir::mir::ty::optional::MirTypeOptional {
                inner: Box::new(substitute_type(&opt.inner, substitution_map)?),
            },
        )),
        MirType::GeneralList(list) => Ok(MirType::GeneralList(
            crate::codegen::ir::mir::ty::general_list::MirTypeGeneralList {
                inner: Box::new(substitute_type(&list.inner, substitution_map)?),
            },
        )),
        MirType::Record(record) => Ok(MirType::Record(
            crate::codegen::ir::mir::ty::record::MirTypeRecord {
                inner: record.inner.clone(),
                values: record
                    .values
                    .iter()
                    .map(|v| substitute_type(v, substitution_map))
                    .collect::<anyhow::Result<Vec<_>>>()?
                    .into_boxed_slice(),
            },
        )),
        // For enum/struct refs, check if they refer to the same generic template being instantiated
        // This handles recursive generics like Tree<T> containing Tree<T>
        MirType::EnumRef(enum_ref) => {
            // TODO: Handle recursive generic enum references
            // For now, just clone (will need to check if this enum ref needs substitution)
            Ok(MirType::EnumRef(enum_ref.clone()))
        }
        MirType::StructRef(struct_ref) => {
            // TODO: Handle recursive generic struct references
            Ok(MirType::StructRef(struct_ref.clone()))
        }
        // Other types don't contain generics, so just clone
        MirType::DartFn(_)
        | MirType::DartOpaque(_)
        | MirType::Delegate(_)
        | MirType::Dynamic(_)
        | MirType::Primitive(_)
        | MirType::PrimitiveList(_)
        | MirType::RustAutoOpaqueImplicit(_)
        | MirType::RustOpaque(_)
        | MirType::TraitDef(_) => Ok(ty.clone()),
    }
}

