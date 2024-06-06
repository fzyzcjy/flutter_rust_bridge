use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::type_parser::misc::convert_ident_str;
use crate::codegen::parser::mir::type_parser::TypeParserWithContext;
use anyhow::Context;
use syn::Type;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type(&mut self, ty: &Type) -> anyhow::Result<MirType> {
        let resolve_ty = self.resolve_alias(ty);

        Ok(match resolve_ty.clone() {
            Type::Path(x) => self.parse_type_path(&x)?,
            Type::Array(x) => self.parse_type_array(&x)?,
            Type::Slice(x) => self.parse_type_slice(&x)?,
            Type::Tuple(x) => self.parse_type_tuple(&x)?,
            // TODO
            // Type::TraitObject(x) => self.parse_type_trait_object(&x)?,
            Type::ImplTrait(x) => self
                .parse_type_impl_trait_dart_fn(&x)
                .context("when trying to parse DartFn")?,
            _ => self.parse_type_rust_auto_opaque_implicit(None, &resolve_ty, None, None)?,
        })
    }

    fn resolve_alias(&self, ty: &Type) -> Type {
        self.get_alias_type(ty).unwrap_or(ty).clone()
    }

    fn get_alias_type(&self, ty: &Type) -> Option<&Type> {
        convert_ident_str(ty).and_then(|key| self.inner.src_types.get(&key))
    }
}
