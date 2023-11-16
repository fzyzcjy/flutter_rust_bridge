use crate::codegen::parser::type_parser::misc::convert_ident_str;
use crate::codegen::parser::type_parser::TypeParser;
use syn::Type;

impl<'a> TypeParser<'a> {
    pub(crate) fn resolve_alias<'b: 'a>(&self, ty: &'b Type) -> &Type {
        self.get_alias_type(ty).unwrap_or(ty)
    }

    pub(crate) fn get_alias_type(&self, ty: &Type) -> Option<&Type> {
        convert_ident_str(ty).and_then(|key| self.src_types.get(&key))
    }
}
