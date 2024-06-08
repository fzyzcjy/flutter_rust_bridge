use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::parser::ty::misc::convert_ident_str;
use crate::codegen::parser::mir::parser::ty::TypeParserWithContext;
use anyhow::Context;
use syn::Type;
use crate::codegen::ir::mir::ty::delegate::{MirTypeDelegate, MirTypeDelegateProxyEnum};

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type(&mut self, ty: &Type) -> anyhow::Result<MirType> {
        let resolve_ty = self.resolve_alias(ty);
        let output = self.parse_type_inner(&resolve_ty)?;
        let output = self.transform_parsed_type(output)?;
        Ok(output)
    }

    fn parse_type_inner(&mut self, ty: &Type) -> anyhow::Result<MirType> {
        Ok(match ty.clone() {
            Type::Path(x) => self.parse_type_path(&x)?,
            Type::Array(x) => self.parse_type_array(&x)?,
            Type::Slice(x) => self.parse_type_slice(&x)?,
            Type::Tuple(x) => self.parse_type_tuple(&x)?,
            // Type::TraitObject(x) => self.parse_type_trait_object(&x)?,
            Type::ImplTrait(x) => self
                .parse_type_impl_trait_dart_fn(&x)
                .context("when trying to parse DartFn")?,
            _ => self.parse_type_rust_auto_opaque_implicit(None, &ty, None, None)?,
        })
    }

    fn resolve_alias(&self, ty: &Type) -> Type {
        self.get_alias_type(ty).unwrap_or(ty).clone()
    }

    fn get_alias_type(&self, ty: &Type) -> Option<&Type> {
        convert_ident_str(ty).and_then(|key| self.inner.src_types.get(&key))
    }

    fn transform_parsed_type(&self, ty: MirType) -> anyhow::Result<MirType> {
        if self.inner.proxied_types.contains(&ty) {
            return Ok(MirType::Delegate(MirTypeDelegate::ProxyEnum(MirTypeDelegateProxyEnum {
                inner: Box::new(ty),
            })));
        }

        Ok(ty)
    }
}
