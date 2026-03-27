use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::parser::ty::misc::convert_ident_str;
use crate::codegen::parser::mir::parser::ty::{TypeParserParsingContext, TypeParserWithContext};
use crate::utils::syn_utils::ty_to_string;
use anyhow::Context;
use syn::Type;

impl TypeParserWithContext<'_, '_, '_> {
    pub(crate) fn parse_type(&mut self, ty: &Type) -> anyhow::Result<MirType> {
        let alias_name = convert_ident_str(ty).and_then(|key| {
            if self.inner.src_types.contains_key(&key) {
                Some(key)
            } else {
                None
            }
        });

        let resolve_ty = self.resolve_alias(ty);

        let context_with_alias = if alias_name.is_some() {
            self.context.with_type_alias(true)
        } else {
            self.context.clone()
        };

        let mut parser_with_context = TypeParserWithContext {
            inner: self.inner,
            context: &context_with_alias,
        };

        let ans = parser_with_context.parse_type_inner(&resolve_ty, alias_name.as_deref())?;
        log::debug!(
            "TypeParserWithContext.parse_type ty={} ans={ans:?}",
            ty_to_string(ty)
        );
        Ok(ans)
    }

    fn parse_type_inner(
        &mut self,
        ty: &Type,
        alias_name: Option<&str>,
    ) -> anyhow::Result<MirType> {
        let ans = match ty.clone() {
            Type::Path(x) => self.parse_type_path(&x, alias_name)?,
            Type::Array(x) => self.parse_type_array(&x)?,
            Type::Slice(x) => self.parse_type_slice(&x)?,
            Type::Tuple(x) => self.parse_type_tuple(&x)?,
            Type::TraitObject(x) => self.parse_type_trait_object(&x)?,
            Type::ImplTrait(x) => self
                .parse_type_impl_trait_dart_fn(&x)
                .context("when trying to parse DartFn")?,
            _ => self.parse_type_rust_auto_opaque_implicit(None, ty, None, None)?,
        };
        Ok(ans)
    }

    fn parse_type_path(
        &mut self,
        type_path: &syn::TypePath,
        alias_name: Option<&str>,
    ) -> anyhow::Result<MirType> {
        match &type_path {
            syn::TypePath { qself: None, path } => {
                self.parse_type_path_core(type_path, path, alias_name)
            }
            syn::TypePath {
                qself: Some(syn::QSelf { ty, .. }),
                ..
            } => anyhow::bail!(
                "qself \"<{}>\" in \"{}\", and all qself syntax, is unsupported",
                quote::ToTokens::to_token_stream(ty),
                quote::ToTokens::to_token_stream(type_path)
            ),
        }
    }

    fn parse_type_path_core(
        &mut self,
        type_path: &syn::TypePath,
        path: &syn::Path,
        alias_name: Option<&str>,
    ) -> anyhow::Result<MirType> {
        use crate::codegen::parser::mir::parser::ty::path_data::extract_path_data;
        use crate::codegen::parser::mir::parser::ty::unencodable::splay_segments;
        use crate::codegen::ir::mir::ty::generic::MirTypeGeneric;

        let segments = extract_path_data(path)?;
        let splayed_segments = splay_segments(&segments);

        // Check if this is a generic type parameter (e.g., T in Change<T>)
        // Generic parameters are simple identifiers with no path segments and no type arguments
        if splayed_segments.len() == 1 {
            if let Some((ident, args)) = splayed_segments.last() {
                // Generic type parameter: single identifier, no type arguments
                if args.is_empty() && self.context.current_generic_params.contains(&ident.to_string()) {
                    return Ok(MirType::Generic(MirTypeGeneric {
                        param_name: ident.to_string(),
                    }));
                }
            }
        }

        if let Some(last_segment) = splayed_segments.last() {
            if let Some(ans) = self.parse_type_path_data_custom_ser_des(last_segment)? {
                return Ok(ans);
            }
            if let Some(ans) = self.parse_type_path_data_primitive(last_segment)? {
                return Ok(ans);
            }
            if let Some(ans) =
                self.parse_type_path_data_concrete(last_segment, &splayed_segments)?
            {
                return Ok(ans);
            }
            // Pass alias_name to struct parsing
            if let Some(ans) = self.parse_type_path_data_struct_with_alias(path, last_segment, None, alias_name)? {
                return Ok(ans);
            }
            // Pass alias_name to enum parsing
            if let Some(ans) = self.parse_type_path_data_enum_with_alias(path, last_segment, alias_name)? {
                return Ok(ans);
            }
            if let Some(ans) = self.parse_type_path_data_trait(last_segment)? {
                return Ok(ans);
            }
            if let Some(ans) = self.parse_type_path_data_rust_opaque(last_segment)? {
                return Ok(ans);
            }
            if let Some(ans) = self.parse_type_path_data_rust_auto_opaque_explicit(last_segment)? {
                return Ok(ans);
            }
            if let Some(ans) = self.parse_type_path_data_optional(type_path, last_segment)? {
                return Ok(ans);
            }
        }

        self.parse_type_rust_auto_opaque_implicit(
            None,
            &syn::Type::Path(type_path.to_owned()),
            None,
            None,
        )
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

    fn resolve_alias(&self, ty: &Type) -> Type {
        self.get_alias_type(ty).unwrap_or(ty).clone()
    }

    fn get_alias_type(&self, ty: &Type) -> Option<&Type> {
        convert_ident_str(ty).and_then(|key| self.inner.src_types.get(&key))
    }
}
