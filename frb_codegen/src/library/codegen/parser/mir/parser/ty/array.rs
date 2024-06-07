use crate::codegen::ir::mir::ty::delegate::{
    MirTypeDelegate, MirTypeDelegateArray, MirTypeDelegateArrayMode,
};
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::ir::mir::ty::MirType::{Delegate, Primitive};
use crate::codegen::parser::mir::parser::ty::TypeParserWithContext;
use crate::utils::namespace::Namespace;
use anyhow::bail;
use std::collections::HashMap;
use syn::Expr;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_array(
        &mut self,
        type_array: &syn::TypeArray,
    ) -> anyhow::Result<MirType> {
        let length: usize = match &type_array.len {
            Expr::Lit(lit) => match &lit.lit {
                syn::Lit::Int(x) => x.base10_parse()?,
                // This will stop the whole generator and tell the users, so we do not care about testing it
                // frb-coverage:ignore-start
                _ => bail!("Cannot parse array length"),
            },
            _ => bail!("Cannot parse array length"),
            // frb-coverage:ignore-end
        };

        let mode = match self.parse_type(&type_array.elem)? {
            Primitive(primitive) => MirTypeDelegateArrayMode::Primitive(primitive),
            others => MirTypeDelegateArrayMode::General(Box::new(others)),
        };

        let namespace = (self.inner.array_parser_info.namespace_of_parsed_types)
            .entry((mode.clone(), length))
            .or_insert(self.context.initiated_namespace.clone())
            .to_owned();

        Ok(Delegate(MirTypeDelegate::Array(MirTypeDelegateArray {
            namespace,
            length,
            mode,
        })))
    }
}

#[derive(Clone, Debug, Default)]
pub(super) struct ArrayParserInfo {
    namespace_of_parsed_types: HashMap<(MirTypeDelegateArrayMode, usize), Namespace>,
}
