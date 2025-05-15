use crate::codegen::parser::mir::parser::ty::TypeParserWithContext;
use crate::utils::namespace::Namespace;

impl TypeParserWithContext<'_, '_, '_> {
    pub(crate) fn parse_namespace_by_name(&mut self, raw: &str) -> Option<Namespace> {
        self.parse_struct_namespace(raw)
            .or_else(|| self.parse_enum_namespace(raw))
    }
}
