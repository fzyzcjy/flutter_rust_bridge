use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::type_parser::misc::convert_ident_str;
use crate::codegen::parser::mir::type_parser::TypeParserWithContext;
use anyhow::Context;
use syn::Type;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type(&mut self, ty: &Type) -> anyhow::Result<MirType> {
        let resolve_ty = self.resolve_alias(ty);

        Ok(match resolve_ty.clone() {
            Type::Path(path) => self.parse_type_path(&path)?,
            Type::Array(type_array) => self.parse_type_array(&type_array)?,
            Type::Slice(type_slice) => self.parse_type_slice(&type_slice)?,
            Type::Tuple(type_tuple) => self.parse_type_tuple(&type_tuple)?,
            Type::TraitObject(type_trait_object) => {
                self.parse_type_trait_object(&type_trait_object)?
            }
            Type::ImplTrait(type_impl_trait) => self
                .parse_type_impl_trait_dart_fn(&type_impl_trait)
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
