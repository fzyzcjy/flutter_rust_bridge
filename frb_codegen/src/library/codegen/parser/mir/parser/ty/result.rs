use crate::codegen::ir::mir::ty::delegate::MirTypeDelegate;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::ir::mir::ty::MirType::{EnumRef, StructRef};
use crate::codegen::parser::mir::parser::ty::unencodable::splay_segments;
use crate::codegen::parser::mir::parser::ty::{TypeParser, TypeParserParsingContext};
use anyhow::Context;
use syn::{GenericArgument, PathArguments, Type, TypePath};

#[allow(clippy::single_match)] // deliberate do so to ensure style consistency
pub(crate) fn parse_type_maybe_result(
    mir: &MirType,
    type_parser: &mut TypeParser,
    context: &TypeParserParsingContext,
) -> anyhow::Result<ResultTypeInfo> {
    if let MirType::RustAutoOpaqueImplicit(inner) = mir {
        match splay_segments(&inner.raw.segments).last() {
            Some((type_name, args)) => {
                // Check if this is a Result type directly
                if type_name == &"Result" {
                    // Check if this might be a generic type alias like `type Result<T> = std::result::Result<T, MyError>`
                    // If we only have one arg, try to resolve the error type from the alias definition
                    let resolved_args = if args.len() == 1 {
                        resolve_generic_result_alias_args(type_parser, type_name, args)
                    } else {
                        None
                    };

                    let args_to_parse = resolved_args.as_deref().unwrap_or(args);
                    return parse_type_result(
                        &(args_to_parse.iter())
                            .map(|arg| type_parser.parse_type(arg, context))
                            .collect::<anyhow::Result<Vec<_>>>()?,
                    );
                }
            }
            _ => {}
        }
    }

    Ok(ResultTypeInfo {
        ok_output: mir.clone(),
        error_output: None,
    })
}

/// Attempts to resolve the error type from a generic type alias definition.
/// For example, if we have `type Result<T> = std::result::Result<T, MyError>` and
/// the function returns `Result<Foo>`, this will return `[Foo, MyError]`.
fn resolve_generic_result_alias_args<'a>(
    type_parser: &'a TypeParser,
    type_name: &str,
    provided_args: &'a [Type],
) -> Option<Vec<Type>> {
    let alias = type_parser.src_generic_types.get(type_name)?;

    // Extract the target type's generic arguments
    // The target should be something like `std::result::Result<T, MyError>`
    let target_args = extract_type_path_args(&alias.target)?;

    // We need at least 2 args (ok type and error type) in the target
    if target_args.len() < 2 {
        return None;
    }

    // Build a mapping from generic params to provided args
    // e.g., if generics is `<T>` and provided_args is `[Foo]`, then T -> Foo
    let generics = alias.generics.as_ref()?;
    let param_names: Vec<String> = generics
        .params
        .iter()
        .filter_map(|p| {
            if let syn::GenericParam::Type(tp) = p {
                Some(tp.ident.to_string())
            } else {
                None
            }
        })
        .collect();

    if param_names.len() != provided_args.len() {
        return None;
    }

    let param_map: std::collections::HashMap<&str, &Type> = param_names
        .iter()
        .zip(provided_args.iter())
        .map(|(name, ty)| (name.as_str(), ty))
        .collect();

    // Substitute generic params in target_args with provided_args
    let resolved: Vec<Type> = target_args
        .iter()
        .map(|arg| substitute_type_param(arg, &param_map))
        .collect();

    Some(resolved)
}

/// Extract generic arguments from a Type::Path
fn extract_type_path_args(ty: &Type) -> Option<Vec<Type>> {
    if let Type::Path(TypePath { path, .. }) = ty {
        // Get the last segment (e.g., `Result` from `std::result::Result`)
        let last_segment = path.segments.last()?;
        if let PathArguments::AngleBracketed(args) = &last_segment.arguments {
            let types: Vec<Type> = args
                .args
                .iter()
                .filter_map(|arg| {
                    if let GenericArgument::Type(t) = arg {
                        Some(t.clone())
                    } else {
                        None
                    }
                })
                .collect();
            return Some(types);
        }
    }
    None
}

/// Substitute type parameters in a type with concrete types from the map
fn substitute_type_param(ty: &Type, param_map: &std::collections::HashMap<&str, &Type>) -> Type {
    if let Type::Path(TypePath { qself, path }) = ty {
        // Check if this is a simple type parameter reference like `T`
        if qself.is_none() && path.segments.len() == 1 {
            let segment = &path.segments[0];
            if segment.arguments.is_empty() {
                let ident = segment.ident.to_string();
                if let Some(&replacement) = param_map.get(ident.as_str()) {
                    return replacement.clone();
                }
            }
        }
    }
    // If not a substitutable param, return as-is
    ty.clone()
}

fn parse_type_result(args: &[MirType]) -> anyhow::Result<ResultTypeInfo> {
    let ok_output = args
        .first()
        .with_context(|| "invalid number of args".to_string())?;

    let is_anyhow = args.len() == 1
        || args.iter().any(|x| {
            if let MirType::RustAutoOpaqueImplicit(inner) = x {
                // Indeed `anyhow :: Error`, but we stripped the prefixes
                return inner.raw.string.with_static_lifetime().trim() == "Error";
            }
            false
        });

    let error_output = if is_anyhow {
        Some(MirType::Delegate(MirTypeDelegate::AnyhowException))
    } else {
        args.last().cloned()
    };

    let error_output = error_output.map(set_is_exception_flag);

    Ok(ResultTypeInfo {
        ok_output: ok_output.clone(),
        error_output,
    })
}

pub(crate) struct ResultTypeInfo {
    pub ok_output: MirType,
    pub error_output: Option<MirType>,
}

fn set_is_exception_flag(mut ty: MirType) -> MirType {
    match &mut ty {
        StructRef(ref mut inner) => {
            inner.is_exception = true;
        }
        EnumRef(ref mut inner) => {
            inner.is_exception = true;
        }
        _ => {}
    }
    ty
}
