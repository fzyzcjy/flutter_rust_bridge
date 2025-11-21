use crate::codegen::ir::mir::field::MirField;
use crate::codegen::ir::mir::ty::delegate::MirTypeDelegate;
use crate::codegen::ir::mir::ty::enumeration::{MirEnum, MirEnumVariant, MirVariantKind};
use crate::codegen::ir::mir::ty::structure::MirStruct;
use crate::codegen::ir::mir::ty::MirType;
use crate::utils::namespace::NamespacedName;
use std::collections::HashMap;

fn build_substitution_map(
    generic_params: &[String],
    type_args: &[MirType],
) -> anyhow::Result<HashMap<String, MirType>> {
    assert!(
        generic_params.len() == type_args.len(),
        "Mismatch in generic parameter count: template has {}, but {} type arguments provided",
        generic_params.len(),
        type_args.len()
    );

    Ok(generic_params
        .iter()
        .zip(type_args.iter())
        .map(|(param_name, concrete_type)| (param_name.clone(), concrete_type.clone()))
        .collect())
}

fn substitute_fields(
    fields: &[MirField],
    substitution_map: &HashMap<String, MirType>,
) -> anyhow::Result<Vec<MirField>> {
    fields
        .iter()
        .map(|field| {
            Ok(MirField {
                ty: substitute_type(&field.ty, substitution_map)?,
                name: field.name.clone(),
                is_final: field.is_final,
                is_rust_public: field.is_rust_public,
                comments: field.comments.clone(),
                default: field.default.clone(),
                settings: field.settings.clone(),
            })
        })
        .collect()
}

fn update_wrapper_name(
    template_wrapper: &Option<String>,
    template_name: &NamespacedName,
    concrete_name: &NamespacedName,
) -> Option<String> {
    match template_wrapper {
        Some(wrapper) => {
            let template_rust = template_name.rust_style();
            if wrapper.contains(&template_rust) {
                Some(wrapper.replace(&template_rust, &concrete_name.rust_style()))
            } else {
                template_wrapper.clone()
            }
        }
        None => None,
    }
}

fn compute_ignore(template_ignore: bool, is_from_type_alias: bool) -> bool {
    if is_from_type_alias {
        false
    } else {
        template_ignore
    }
}

pub(crate) fn instantiate_enum(
    template: &MirEnum,
    concrete_name: NamespacedName,
    type_args: &[MirType],
    is_from_type_alias: bool,
) -> anyhow::Result<MirEnum> {
    let substitution_map = build_substitution_map(&template.generic_params, type_args)?;

    let substituted_variants = template
        .variants
        .iter()
        .map(|variant| {
            let substituted_kind = match &variant.kind {
                MirVariantKind::Value => MirVariantKind::Value,
                MirVariantKind::Struct(struct_data) => {
                    MirVariantKind::Struct(MirStruct {
                        name: struct_data.name.clone(),
                        wrapper_name: struct_data.wrapper_name.clone(),
                        fields: substitute_fields(&struct_data.fields, &substitution_map)?,
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
                    })
                }
            };
            let wrapper_name_str = variant.wrapper_name.rust_style(true);
            let updated_wrapper_name = if wrapper_name_str.starts_with(&template.name.name) {
                let variant_part = &wrapper_name_str[template.name.name.len()..];
                format!("{}{}", concrete_name.name, variant_part)
            } else {
                wrapper_name_str
            };

            Ok(MirEnumVariant {
                name: variant.name.clone(),
                wrapper_name: crate::codegen::ir::mir::ident::MirIdent::new(
                    updated_wrapper_name,
                    None,
                ),
                comments: variant.comments.clone(),
                kind: substituted_kind,
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(MirEnum {
        name: concrete_name.clone(),
        wrapper_name: update_wrapper_name(&template.wrapper_name, &template.name, &concrete_name),
        comments: template.comments.clone(),
        variants: substituted_variants,
        mode: template.mode,
        ignore: compute_ignore(template.ignore, is_from_type_alias),
        needs_json_serializable: template.needs_json_serializable,
        generic_params: vec![],
        is_generic_template: false,
    })
}

pub(crate) fn instantiate_struct(
    template: &MirStruct,
    concrete_name: NamespacedName,
    type_args: &[MirType],
    is_from_type_alias: bool,
) -> anyhow::Result<MirStruct> {
    let substitution_map = build_substitution_map(&template.generic_params, type_args)?;

    Ok(MirStruct {
        name: concrete_name.clone(),
        wrapper_name: update_wrapper_name(&template.wrapper_name, &template.name, &concrete_name),
        fields: substitute_fields(&template.fields, &substitution_map)?,
        is_fields_named: template.is_fields_named,
        dart_metadata_raw: template.dart_metadata_raw.clone(),
        ignore: compute_ignore(template.ignore, is_from_type_alias),
        needs_json_serializable: template.needs_json_serializable,
        generate_hash: template.generate_hash,
        generate_eq: template.generate_eq,
        ui_state: template.ui_state,
        comments: template.comments.clone(),
        generic_params: vec![],
        is_generic_template: false,
    })
}

/// Recursively substitute generic type parameters with concrete types
fn substitute_type(
    ty: &MirType,
    substitution_map: &HashMap<String, MirType>,
) -> anyhow::Result<MirType> {
    match ty {
        MirType::Generic(generic) => substitution_map
            .get(&generic.param_name)
            .cloned()
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Generic parameter '{}' not found in substitution map",
                    generic.param_name
                )
            }),
        MirType::Boxed(boxed) => Ok(MirType::Boxed(
            crate::codegen::ir::mir::ty::boxed::MirTypeBoxed {
                exist_in_real_api: boxed.exist_in_real_api,
                inner: Box::new(substitute_type(&boxed.inner, substitution_map)?),
            },
        )),
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
        MirType::Delegate(delegate) => substitute_delegate(delegate, substitution_map),
        // EnumRef/StructRef are identifiers pointing into the object pool.
        // During template parsing, nested generic types like `Bar<T>` inside `Foo<T>`
        // are already instantiated by the parser (producing e.g. `Bar_T`).
        // Recursive types (e.g. Tree<T> containing Tree<T>) are not yet supported;
        // they would require re-instantiation here with the concrete type args.
        MirType::EnumRef(_)
        | MirType::StructRef(_)
        | MirType::DartFn(_)
        | MirType::DartOpaque(_)
        | MirType::Dynamic(_)
        | MirType::Primitive(_)
        | MirType::PrimitiveList(_)
        | MirType::RustAutoOpaqueImplicit(_)
        | MirType::RustOpaque(_)
        | MirType::TraitDef(_) => Ok(ty.clone()),
    }
}

fn substitute_delegate(
    delegate: &MirTypeDelegate,
    substitution_map: &HashMap<String, MirType>,
) -> anyhow::Result<MirType> {
    use crate::codegen::ir::mir::ty::delegate::*;
    let substituted = match delegate {
        MirTypeDelegate::Map(map) => MirTypeDelegate::Map(MirTypeDelegateMap {
            key: Box::new(substitute_type(&map.key, substitution_map)?),
            value: Box::new(substitute_type(&map.value, substitution_map)?),
            hasher: map.hasher.clone(),
            element_delegate: crate::codegen::ir::mir::ty::record::MirTypeRecord {
                inner: map.element_delegate.inner.clone(),
                values: map
                    .element_delegate
                    .values
                    .iter()
                    .map(|v| substitute_type(v, substitution_map))
                    .collect::<anyhow::Result<Vec<_>>>()?
                    .into_boxed_slice(),
            },
        }),
        MirTypeDelegate::Set(set) => MirTypeDelegate::Set(MirTypeDelegateSet {
            inner: Box::new(substitute_type(&set.inner, substitution_map)?),
            hasher: set.hasher.clone(),
        }),
        MirTypeDelegate::StreamSink(sink) => {
            MirTypeDelegate::StreamSink(MirTypeDelegateStreamSink {
                inner_ok: Box::new(substitute_type(&sink.inner_ok, substitution_map)?),
                inner_err: Box::new(substitute_type(&sink.inner_err, substitution_map)?),
                codec: sink.codec,
            })
        }
        MirTypeDelegate::Array(arr) => MirTypeDelegate::Array(MirTypeDelegateArray {
            namespace: arr.namespace.clone(),
            length: arr.length,
            mode: match &arr.mode {
                MirTypeDelegateArrayMode::General(inner) => {
                    MirTypeDelegateArrayMode::General(Box::new(substitute_type(
                        inner,
                        substitution_map,
                    )?))
                }
                MirTypeDelegateArrayMode::Primitive(p) => {
                    MirTypeDelegateArrayMode::Primitive(p.clone())
                }
            },
        }),
        _ => return Ok(MirType::Delegate(delegate.clone())),
    };
    Ok(MirType::Delegate(substituted))
}

/// Checks whether any generic arguments are concrete types (not just type parameters).
/// Returns false if all args are generic type parameters from `current_generic_params`.
pub(crate) fn has_concrete_generic_args(
    generic_args: &[syn::Type],
    current_generic_params: &[String],
) -> bool {
    generic_args.iter().any(|arg| {
        !matches!(arg, syn::Type::Path(syn::TypePath { path, .. }) if
            path.segments.len() == 1 &&
            current_generic_params.contains(&path.segments[0].ident.to_string()))
    })
}

/// Strips generic arguments from the last segment of a path.
pub(crate) fn strip_path_generic_args(path: &syn::Path) -> syn::Path {
    let mut stripped = path.clone();
    if let Some(last_seg) = stripped.segments.last_mut() {
        last_seg.arguments = syn::PathArguments::None;
    }
    stripped
}

/// Generates a concrete name for a generic instantiation, using either
/// the provided alias name or auto-generating from type argument names.
pub(crate) fn generate_concrete_name(
    namespace: crate::utils::namespace::Namespace,
    base_name: &str,
    alias_name: Option<&str>,
    type_args: &[MirType],
) -> NamespacedName {
    if let Some(alias) = alias_name {
        return NamespacedName::new(namespace, alias.to_string());
    }
    let type_args_str: Vec<String> = type_args
        .iter()
        .map(|ty| {
            crate::codegen::parser::mir::parser::ty::enumeration::generate_type_name_for_auto_naming(
                ty,
            )
        })
        .collect();
    let auto_name = if type_args_str.is_empty() {
        base_name.to_string()
    } else {
        format!("{}_{}", base_name, type_args_str.join("_"))
    };
    log::info!(
        "Auto-generating name for generic instantiation: {} -> {}",
        base_name,
        auto_name
    );
    NamespacedName::new(namespace, auto_name)
}
