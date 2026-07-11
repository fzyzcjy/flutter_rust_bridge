use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::parser::ty::generic_type_alias::{
    extract_generic_type_args, substitute_type_params,
};
use crate::codegen::parser::mir::parser::ty::misc::convert_ident_str;
use crate::codegen::parser::mir::parser::ty::{TypeParserParsingContext, TypeParserWithContext};
use crate::utils::syn_utils::ty_to_string;
use anyhow::Context;
use syn::Type;

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
        self.resolve_alias_inner(ty, 0)
    }

    fn resolve_alias_inner(&self, ty: &Type, depth: usize) -> Type {
        // Guard against pathological or cyclic generic aliases, such as
        // `type A<T> = B<T>; type B<T> = A<T>;`.
        const MAX_RESOLVE_DEPTH: usize = 64;
        if depth >= MAX_RESOLVE_DEPTH {
            // Only reachable via a deliberately cyclic alias graph, which no real
            // FRB workflow produces; excluded from coverage instead of forcing a
            // brittle full-parser-context test.
            // frb-coverage:ignore-start
            return ty.clone();
            // frb-coverage:ignore-end
        }

        // Generic alias used at a call site, e.g. `AppResult<MyDto>` is expanded
        // to `Result<MyDto, AppError>`. Recurse so chained generic aliases expand
        // to a fixed point before the normal type parsing kicks in.
        if let Some(substituted) = self.substitute_generic_alias(ty) {
            return self.resolve_alias_inner(&substituted, depth + 1);
        }

        // Plain (non-generic) alias, already pre-resolved by the HIR transformer.
        self.get_alias_type(ty).unwrap_or(ty).clone()
    }

    fn substitute_generic_alias(&self, ty: &Type) -> Option<Type> {
        let key = convert_ident_str(ty)?;
        let template = self.inner.src_generic_type_aliases.get(&key)?;
        let args = extract_generic_type_args(ty)?;
        substitute_type_params(&template.type_params, &template.target, &args)
    }

    fn get_alias_type(&self, ty: &Type) -> Option<&Type> {
        convert_ident_str(ty).and_then(|key| self.inner.src_types.get(&key))
    }
}
