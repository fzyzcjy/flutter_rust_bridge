use std::collections::HashMap;
use syn::visit_mut::VisitMut;
use syn::{GenericArgument, PathArguments, Type, TypePath};

/// Identifiers that are handled specially by the code generator and therefore
/// must never be treated as substitutable generic aliases. In particular a
/// user-defined `type Result<T> = ...` (see #1710) must keep flowing through the
/// built-in fallible-return detection instead of being expanded here.
pub(crate) fn is_reserved_generic_alias_ident(ident: &str) -> bool {
    ident == "Result"
}

/// Substitute the type parameters of a generic alias template with the concrete
/// arguments found at a use site.
///
/// For `type AppResult<T> = Result<T, AppError>` (template `type_params = ["T"]`,
/// `target = Result<T, AppError>`) applied to `AppResult<MyDto>`
/// (`args = [MyDto]`), this returns `Result<MyDto, AppError>`.
///
/// Returns `None` when the number of arguments does not match the number of type
/// parameters (e.g. partial application), in which case the caller should leave
/// the original type untouched.
pub(crate) fn substitute_type_params(
    type_params: &[String],
    target: &Type,
    args: &[Type],
) -> Option<Type> {
    if args.len() != type_params.len() {
        return None;
    }

    let mapping: HashMap<String, Type> = (type_params.iter().cloned())
        .zip(args.iter().cloned())
        .collect();

    let mut output = target.clone();
    ParamSubstitutor { mapping }.visit_type_mut(&mut output);
    Some(output)
}

/// Extract the angle-bracketed generic type arguments from the last segment of a
/// path type, e.g. `[MyDto]` for `AppResult<MyDto>`. Returns `None` if the type
/// is not a path or has no angle-bracketed arguments.
pub(crate) fn extract_generic_type_args(ty: &Type) -> Option<Vec<Type>> {
    if let Type::Path(TypePath { qself: None, path }) = ty {
        if let Some(segment) = path.segments.last() {
            if let PathArguments::AngleBracketed(angle_bracketed) = &segment.arguments {
                let types = (angle_bracketed.args.iter())
                    .filter_map(|arg| match arg {
                        GenericArgument::Type(t) => Some(t.clone()),
                        _ => None,
                    })
                    .collect();
                return Some(types);
            }
        }
    }
    None
}

struct ParamSubstitutor {
    mapping: HashMap<String, Type>,
}

impl VisitMut for ParamSubstitutor {
    fn visit_type_mut(&mut self, node: &mut Type) {
        // Replace a bare identifier (e.g. `T`) that matches a type parameter with
        // the concrete argument. Only single-segment paths without their own
        // generic arguments are considered type-parameter references.
        if let Type::Path(TypePath { qself: None, path }) = node {
            if path.segments.len() == 1 {
                let segment = &path.segments[0];
                if matches!(segment.arguments, PathArguments::None) {
                    if let Some(replacement) = self.mapping.get(&segment.ident.to_string()) {
                        *node = replacement.clone();
                        // Do not recurse into the freshly substituted type.
                        return;
                    }
                }
            }
        }

        syn::visit_mut::visit_type_mut(self, node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    fn parse(s: &str) -> Type {
        syn::parse_str(s).unwrap()
    }

    #[test]
    fn test_substitute_simple_result_alias() {
        // type AppResult<T> = Result<T, AppError>; use site AppResult<MyDto>
        let target: Type = parse_quote!(Result<T, AppError>);
        let args = vec![parse("MyDto")];
        let output = substitute_type_params(&["T".to_owned()], &target, &args).unwrap();
        assert_eq!(output, parse_quote!(Result<MyDto, AppError>));
    }

    #[test]
    fn test_substitute_multiple_params() {
        // type Pair<A, B> = (B, A); use site Pair<i32, String>
        let target: Type = parse_quote!((B, A));
        let args = vec![parse("i32"), parse("String")];
        let output =
            substitute_type_params(&["A".to_owned(), "B".to_owned()], &target, &args).unwrap();
        assert_eq!(output, parse_quote!((String, i32)));
    }

    #[test]
    fn test_substitute_nested_param() {
        // type Wrap<T> = Vec<Option<T>>; use site Wrap<MyDto>
        let target: Type = parse_quote!(Vec<Option<T>>);
        let args = vec![parse("MyDto")];
        let output = substitute_type_params(&["T".to_owned()], &target, &args).unwrap();
        assert_eq!(output, parse_quote!(Vec<Option<MyDto>>));
    }

    #[test]
    fn test_substitute_arg_count_mismatch_returns_none() {
        let target: Type = parse_quote!(Result<T, AppError>);
        let args = vec![parse("MyDto"), parse("Extra")];
        assert!(substitute_type_params(&["T".to_owned()], &target, &args).is_none());
    }

    #[test]
    fn test_extract_generic_type_args() {
        let args = extract_generic_type_args(&parse("AppResult<MyDto>")).unwrap();
        assert_eq!(args, vec![parse("MyDto")]);
    }

    #[test]
    fn test_extract_generic_type_args_multiple() {
        let args = extract_generic_type_args(&parse("Map<String, i32>")).unwrap();
        assert_eq!(args, vec![parse("String"), parse("i32")]);
    }

    #[test]
    fn test_extract_generic_type_args_none_for_plain_path() {
        assert!(extract_generic_type_args(&parse("PlainType")).is_none());
    }

    #[test]
    fn test_extract_generic_type_args_qualified_path() {
        // Arguments live on the *last* segment of a qualified path.
        let args = extract_generic_type_args(&parse("std::result::Result<MyDto>")).unwrap();
        assert_eq!(args, vec![parse("MyDto")]);
    }

    #[test]
    fn test_extract_generic_type_args_skips_non_type_args() {
        // Lifetime (and const) arguments are not types and are filtered out.
        let args = extract_generic_type_args(&parse("Cow<'a, str>")).unwrap();
        assert_eq!(args, vec![parse("str")]);
    }

    #[test]
    fn test_reserved_ident() {
        assert!(is_reserved_generic_alias_ident("Result"));
        assert!(!is_reserved_generic_alias_ident("AppResult"));
    }
}
