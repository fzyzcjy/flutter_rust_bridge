use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::parser::ty::misc::convert_ident_str;
use crate::codegen::parser::mir::parser::ty::{TypeParserParsingContext, TypeParserWithContext};
use crate::utils::syn_utils::ty_to_string;
use anyhow::Context;
use std::collections::HashMap;
use syn::{GenericArgument, PathArguments, Type, TypePath};

impl TypeParserWithContext<'_, '_, '_> {
    pub(crate) fn parse_type(&mut self, ty: &Type) -> anyhow::Result<MirType> {
        let resolve_ty = self.resolve_alias(ty);
        let ans = self.parse_type_inner(&resolve_ty)?;
        log::debug!(
            "TypeParserWithContext.parse_type ty={} ans={ans:?}",
            ty_to_string(ty)
        );
        Ok(ans)
    }

    pub(crate) fn parse_type_with_context(
        &mut self,
        ty: &Type,
        context_modifier: impl FnOnce(&TypeParserParsingContext) -> TypeParserParsingContext,
    ) -> anyhow::Result<MirType> {
        let new_context = context_modifier(self.context);
        let mut self_with_context = TypeParserWithContext {
            inner: self.inner,
            context: &new_context,
        };
        self_with_context.parse_type(ty)
    }

    fn parse_type_inner(&mut self, ty: &Type) -> anyhow::Result<MirType> {
        Ok(match ty.clone() {
            Type::Path(x) => self.parse_type_path(&x)?,
            Type::Array(x) => self.parse_type_array(&x)?,
            Type::Slice(x) => self.parse_type_slice(&x)?,
            Type::Tuple(x) => self.parse_type_tuple(&x)?,
            Type::TraitObject(x) => self.parse_type_trait_object(&x)?,
            Type::ImplTrait(x) => self
                .parse_type_impl_trait_dart_fn(&x)
                .context("when trying to parse DartFn")?,
            _ => self.parse_type_rust_auto_opaque_implicit(None, ty, None, None)?,
        })
    }

    fn resolve_alias(&self, ty: &Type) -> Type {
        // First try non-generic alias resolution
        if let Some(resolved) = self.get_alias_type(ty) {
            return resolved.clone();
        }
        
        // Then try generic alias resolution
        if let Some(resolved) = self.resolve_generic_alias(ty) {
            return resolved;
        }
        
        ty.clone()
    }

    fn get_alias_type(&self, ty: &Type) -> Option<&Type> {
        convert_ident_str(ty).and_then(|key| self.inner.src_types.get(&key))
    }
    
    /// Resolve a generic type alias like `WResult<Uuid>` to `std::result::Result<Uuid, MyError>`
    fn resolve_generic_alias(&self, ty: &Type) -> Option<Type> {
        let Type::Path(TypePath { qself: None, path }) = ty else {
            return None;
        };
        
        // Get the last segment which has the type name and generic args
        let last_segment = path.segments.last()?;
        let type_name = last_segment.ident.to_string();
        
        // Look up the generic type alias
        let alias = self.inner.src_generic_types.get(&type_name)?;
        
        // Extract provided generic arguments
        let PathArguments::AngleBracketed(provided_args) = &last_segment.arguments else {
            return None;
        };
        let provided_types: Vec<&Type> = provided_args
            .args
            .iter()
            .filter_map(|arg| {
                if let GenericArgument::Type(t) = arg {
                    Some(t)
                } else {
                    None
                }
            })
            .collect();
        
        // Get generic parameter names from the alias definition
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
        
        // Parameter count must match
        if param_names.len() != provided_types.len() {
            return None;
        }
        
        // Build substitution map: T -> actual_type
        let param_map: HashMap<String, &Type> = param_names
            .into_iter()
            .zip(provided_types)
            .collect();
        
        // Substitute in the target type
        Some(substitute_type_params(&alias.target, &param_map))
    }
}

/// Recursively substitute type parameters in a type with concrete types from the map
fn substitute_type_params(ty: &Type, param_map: &HashMap<String, &Type>) -> Type {
    match ty {
        Type::Path(TypePath { qself, path }) => {
            // Check if this is a simple type parameter reference like `T`
            if qself.is_none() && path.segments.len() == 1 {
                let segment = &path.segments[0];
                if segment.arguments.is_empty() {
                    let ident = segment.ident.to_string();
                    if let Some(&replacement) = param_map.get(&ident) {
                        return replacement.clone();
                    }
                }
            }
            
            // Otherwise, recursively substitute in path segments
            let new_segments: syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep> = 
                path.segments
                    .iter()
                    .map(|seg| {
                        let new_args = match &seg.arguments {
                            PathArguments::None => PathArguments::None,
                            PathArguments::AngleBracketed(args) => {
                                let new_args: syn::punctuated::Punctuated<GenericArgument, syn::token::Comma> = 
                                    args.args
                                        .iter()
                                        .map(|arg| match arg {
                                            GenericArgument::Type(t) => {
                                                GenericArgument::Type(substitute_type_params(t, param_map))
                                            }
                                            other => other.clone(),
                                        })
                                        .collect();
                                PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                                    colon2_token: args.colon2_token,
                                    lt_token: args.lt_token,
                                    args: new_args,
                                    gt_token: args.gt_token,
                                })
                            }
                            PathArguments::Parenthesized(args) => {
                                PathArguments::Parenthesized(args.clone())
                            }
                        };
                        syn::PathSegment {
                            ident: seg.ident.clone(),
                            arguments: new_args,
                        }
                    })
                    .collect();
            
            Type::Path(TypePath {
                qself: qself.clone(),
                path: syn::Path {
                    leading_colon: path.leading_colon,
                    segments: new_segments,
                },
            })
        }
        Type::Reference(r) => Type::Reference(syn::TypeReference {
            and_token: r.and_token,
            lifetime: r.lifetime.clone(),
            mutability: r.mutability,
            elem: Box::new(substitute_type_params(&r.elem, param_map)),
        }),
        Type::Tuple(t) => Type::Tuple(syn::TypeTuple {
            paren_token: t.paren_token,
            elems: t.elems.iter().map(|e| substitute_type_params(e, param_map)).collect(),
        }),
        Type::Array(a) => Type::Array(syn::TypeArray {
            bracket_token: a.bracket_token,
            elem: Box::new(substitute_type_params(&a.elem, param_map)),
            semi_token: a.semi_token,
            len: a.len.clone(),
        }),
        Type::Slice(s) => Type::Slice(syn::TypeSlice {
            bracket_token: s.bracket_token,
            elem: Box::new(substitute_type_params(&s.elem, param_map)),
        }),
        // For other types, return as-is
        other => other.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;
    
    #[test]
    fn test_substitute_type_params_simple() {
        let ty: Type = syn::parse_str("T").unwrap();
        let uuid_type: Type = syn::parse_str("Uuid").unwrap();
        let mut param_map = HashMap::new();
        param_map.insert("T".to_string(), &uuid_type);
        
        let result = substitute_type_params(&ty, &param_map);
        assert_eq!(result.to_token_stream().to_string(), "Uuid");
    }
    
    #[test]
    fn test_substitute_type_params_in_result() {
        let ty: Type = syn::parse_str("std::result::Result<T, MyError>").unwrap();
        let uuid_type: Type = syn::parse_str("Uuid").unwrap();
        let mut param_map = HashMap::new();
        param_map.insert("T".to_string(), &uuid_type);
        
        let result = substitute_type_params(&ty, &param_map);
        assert_eq!(
            result.to_token_stream().to_string(),
            "std :: result :: Result < Uuid , MyError >"
        );
    }
    
    #[test]
    fn test_substitute_type_params_nested() {
        let ty: Type = syn::parse_str("Option<Vec<T>>").unwrap();
        let i32_type: Type = syn::parse_str("i32").unwrap();
        let mut param_map = HashMap::new();
        param_map.insert("T".to_string(), &i32_type);
        
        let result = substitute_type_params(&ty, &param_map);
        assert_eq!(
            result.to_token_stream().to_string(),
            "Option < Vec < i32 > >"
        );
    }
    
    #[test]
    fn test_substitute_type_params_multiple() {
        let ty: Type = syn::parse_str("HashMap<K, V>").unwrap();
        let string_type: Type = syn::parse_str("String").unwrap();
        let i32_type: Type = syn::parse_str("i32").unwrap();
        let mut param_map = HashMap::new();
        param_map.insert("K".to_string(), &string_type);
        param_map.insert("V".to_string(), &i32_type);
        
        let result = substitute_type_params(&ty, &param_map);
        assert_eq!(
            result.to_token_stream().to_string(),
            "HashMap < String , i32 >"
        );
    }
}
